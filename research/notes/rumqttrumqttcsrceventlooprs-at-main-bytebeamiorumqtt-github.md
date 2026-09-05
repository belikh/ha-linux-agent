---
title: rumqtt/rumqttc/src/eventloop.rs at main · bytebeamio/rumqtt · GitHub
id: rumqttrumqttcsrceventlooprs-at-main-bytebeamiorumqtt-github
tags:
- linux-agent-jupiteros-fleet-15537b
- rumqtt
- rumqttc
- source-code
- reconnect
- primary-source
created: '2026-09-02T04:02:29.646954Z'
updated: '2026-09-05T10:51:21.565427Z'
source: https://github.com/bytebeamio/rumqtt/blob/main/rumqttc/src/eventloop.rs
source_domain: github.com
fetched_at: '2026-09-02T04:02:25.915607Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'Source of rumqttc''s eventloop.rs at main (512 lines / 20KB, fetched from
  GitHub blob page — body extraction includes the full code despite GitHub chrome
  noise). Primary-source evidence for reconnect semantics: (1) EventLoop::poll() —
  ''Continuing to poll will reconnect to the broker if there is a disconnection''
  — on Err from select(), calls self.clean() which moves unacked packets from state
  into the pending VecDeque (drops network, clears keepalive) so QoS1/2 messages are
  republished next session. (2) On reconnect ConnAck: ''If it''s a new session, clear
  the pending packets'' — i.e. session_present=false DISCARDS all queued messages;
  only a persistent session (clean_session=false / MQTT5 session expiry) preserves
  them. (3) clean() doc: ''we recommend setting AsyncClient''s channel capacity to
  0'' — bounded(0) channel — to manage pending list length. (4) Flow control: inflight
  window with collision handling (packet-id collision stops new outgoing requests
  until acked), pending_throttle for replayed pending requests. (5) ConnectionError
  enum: MqttState, NetworkTimeout, FlushTimeout, Io, ConnectionRefused, RequestsDone
  (channel closed), Tls/Websocket/Proxy variants. No re-subscription logic anywhere
  in eventloop.rs — confirms issue #250: subscriptions live in MqttState only via
  broker-side session; with clean sessions they are lost on every reconnect.'
---

rumqtt/rumqttc/src/eventloop.rs at main · bytebeamio/rumqtt · GitHub

Skip to content

Search/

Sign inSign up
Appearance settings

You signed in with another tab or window. Reload to refresh your session.
You signed out in another tab or window. Reload to refresh your session.
You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

Uh oh!

There was an error while loading. Please reload this page.

bytebeamio

/

rumqtt

Public

Notifications
You must be signed in to change notification settings

Fork
342

Star
2.2k

FilesExpand file tree

main

/
eventloop.rsCopy path

Blame
More file actions

Blame
More file actions

Latest commit

HistoryHistory

History

512 lines (460 loc) · 20 KB

main

/
eventloop.rsCopy pathTop

File metadata and controls

Code

Blame

512 lines (460 loc) · 20 KB

Raw
Copy raw file
Download raw file
Open symbols panel
Edit and raw actions

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
96
97
98
99
100
101
102
103
104
105
106
107
108
109
110
111
112
113
114
115
116
117
118
119
120
121
122
123
124
125
126
127
128
129
130
131
132
133
134
135
136
137
138
139
140
141
142
143
144
145
146
147
148
149
150
151
152
153
154
155
156
157
158
159
160
161
162
163
164
165
166
167
168
169
170
171
172
173
174
175
176
177
178
179
180
181
182
183
184
185
186
187
188
189
190
191
192
193
194
195
196
197
198
199
200
201
202
203
204
205
206
207
208
209
210
211
212
213
214
215
216
217
218
219
220
221
222
223
224
225
226
227
228
229
230
231
232
233
234
235
236
237
238
239
240
241
242
243
244
245
246
247
248
249
250
251
252
253
254
255
256
257
258
259
260
261
262
263
264
265
266
267
268
269
270
271
272
273
274
275
276
277
278
279
280
281
282
283
284
285
286
287
288
289
290
291
292
293
294
295
296
297
298
299
300
301
302
303
304
305
306
307
308
309
310
311
312
313
314
315
316
317
318
319
320
321
322
323
324
325
326
327
328
329
330
331
332
333
334
335
336
337
338
339
340
341
342
343
344
345
346
347
348
349
350
351
352
353
354
355
356
357
358
359
360
361
362
363
364
365
366
367
368
369
370
371
372
373
374
375
376
377
378
379
380
381
382
383
384
385
386
387
388
389
390
391
392
393
394
395
396
397
398
399
400
401
402
403
404
405
406
407
408
409
410
411
412
413
414
415
416
417
418
419
420
421
422
423
424
425
426
427
428
429
430
431
432
433
434
435
436
437
438
439
440
441
442
443
444
445
446
447
448
449
450
451
452
453
454
455
456
457
458
459
460
461
462
463
464
465
466
467
468
469
470
471
472
473
474
475
476
477
478
479
480
481
482
483
484
485
486
487
488
489
490
491
492
493
494
495
496
497
498
499
500
501
502
503
504
505
506
507
508
509
510
511
512

use crate::{framed::Network, Transport};

use crate::{Incoming, MqttState, NetworkOptions, Packet, Request, StateError};

use crate::{MqttOptions, Outgoing};

use crate::framed::AsyncReadWrite;

use crate::mqttbytes::v4::*;

use flume::{bounded, Receiver, Sender};

use tokio::net::{lookup_host, TcpSocket, TcpStream};

use tokio::select;

use tokio::time::{self, Instant, Sleep};

use std::collections::VecDeque;

use std::io;

use std::net::SocketAddr;

use std::pin::Pin;

use std::time::Duration;

#[cfg(unix)]

use {std::path::Path, tokio::net::UnixStream};

#[cfg(any(feature = "use-rustls-no-provider", feature = "use-native-tls"))]

use crate::tls;

#[cfg(feature = "websocket")]

use {

crate::websockets::{split_url, validate_response_headers, UrlError},

async_tungstenite::tungstenite::client::IntoClientRequest,

ws_stream_tungstenite::WsStream,

};

#[cfg(feature = "proxy")]

use crate::proxy::ProxyError;

/// Critical errors during eventloop polling

#[derive(Debug, thiserror::Error)]

pub enum ConnectionError {

#[error("Mqtt state: {0}")]

MqttState(#[from] StateError),

#[error("Network timeout")]

NetworkTimeout,

#[error("Flush timeout")]

FlushTimeout,

#[cfg(feature = "websocket")]

#[error("Websocket: {0}")]

Websocket(#[from] async_tungstenite::tungstenite::error::Error),

#[cfg(feature = "websocket")]

#[error("Websocket Connect: {0}")]

WsConnect(#[from] http::Error),

#[cfg(any(feature = "use-rustls-no-provider", feature = "use-native-tls"))]

#[error("TLS: {0}")]

Tls(#[from] tls::Error),

#[error("I/O: {0}")]

Io(#[from] io::Error),

#[error("Connection refused, return code: `{0:?}`")]

ConnectionRefused(ConnectReturnCode),

#[error("Expected ConnAck packet, received: {0:?}")]

NotConnAck(Packet),

#[error("Requests done")]

RequestsDone,

#[cfg(feature = "websocket")]

#[error("Invalid Url: {0}")]

InvalidUrl(#[from] UrlError),

#[cfg(feature = "proxy")]

#[error("Proxy Connect: {0}")]

Proxy(#[from] ProxyError),

#[cfg(feature = "websocket")]

#[error("Websocket response validation error: ")]

ResponseValidation(#[from] crate::websockets::ValidationError),

}

/// Eventloop with all the state of a connection

pub struct EventLoop {

/// Options of the current mqtt connection

pub mqtt_options: MqttOptions,

/// Current state of the connection

pub state: MqttState,

/// Request stream

requests_rx: Receiver<Request>,

/// Requests handle to send requests

pub(crate) requests_tx: Sender<Request>,

/// Pending packets from last session

pub pending: VecDeque<Request>,

/// Network connection to the broker

pub network: Option<Network>,

/// Keep alive time

keepalive_timeout: Option<Pin<Box<Sleep>>>,

pub network_options: NetworkOptions,

}

/// Events which can be yielded by the event loop

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum Event {

Incoming(Incoming),

Outgoing(Outgoing),

}

impl EventLoop {

/// New MQTT `EventLoop`

///

/// When connection encounters critical errors (like auth failure), user has a choice to

/// access and update `options`, `state` and `requests`.

pub fn new(mqtt_options: MqttOptions, cap: usize) -> EventLoop {

let (requests_tx, requests_rx) = bounded(cap);

let pending = VecDeque::new();

let max_inflight = mqtt_options.inflight;

let manual_acks = mqtt_options.manual_acks;

EventLoop {

mqtt_options,

state: MqttState::new(max_inflight, manual_acks),

requests_tx,

requests_rx,

pending,

network: None,

keepalive_timeout: None,

network_options: NetworkOptions::new(),

}

}

/// Last session might contain packets which aren't acked. MQTT says these packets should be

/// republished in the next session. Move pending messages from state to eventloop, drops the

/// underlying network connection and clears the keepalive timeout if any.

///

/// > NOTE: Use only when EventLoop is blocked on network and unable to immediately handle disconnect.

/// > Also, while this helps prevent data loss, the pending list length should be managed properly.

/// > For this reason we recommend setting [`AsycClient`](crate::AsyncClient)'s channel capacity to `0`.

pub fn clean(&mut self) {

self.network = None;

self.keepalive_timeout = None;

self.pending.extend(self.state.clean());

// drain requests from channel which weren't yet received

let mut requests_in_channel: Vec<_> = self.requests_rx.drain().collect();

requests_in_channel.retain(|request| {

match request {

Request::PubAck(_) => false, // Wait for publish retransmission, else the broker could be confused by an unexpected ack

_ => true,

}

});

self.pending.extend(requests_in_channel);

}

/// Yields Next notification or outgoing request and periodically pings

/// the broker. Continuing to poll will reconnect to the broker if there is

/// a disconnection.

/// **NOTE** Don't block this while iterating

pub async fn poll(&mut self) -> Result<Event, ConnectionError> {

if self.network.is_none() {

let (network, connack) = match time::timeout(

Duration::from_secs(self.network_options.connection_timeout()),

connect(&self.mqtt_options, self.network_options.clone()),

)

.await

{

Ok(inner) => inner?,

Err(_) => return Err(ConnectionError::NetworkTimeout),

};

// Last session might contain packets which aren't acked. If it's a new session, clear the pending packets.

if !connack.session_present {

self.pending.clear();

}

self.network = Some(network);

if self.keepalive_timeout.is_none() && !self.mqtt_options.keep_alive.is_zero() {

self.keepalive_timeout = Some(Box::pin(time::sleep(self.mqtt_options.keep_alive)));

}

return Ok(Event::Incoming(Packet::ConnAck(connack)));

}

match self.select().await {

Ok(v) => Ok(v),

Err(e) => {

// MQTT requires that packets pending acknowledgement should be republished on session resume.

// Move pending messages from state to eventloop.

self.clean();

Err(e)

}

}

}

/// Select on network and requests and generate keepalive pings when necessary

async fn select(&mut self) -> Result<Event, ConnectionError> {

let network = self.network.as_mut().unwrap();

// let await_acks = self.state.await_acks;

let inflight_full = self.state.inflight >= self.mqtt_options.inflight;

let collision = self.state.collision.is_some();

let network_timeout = Duration::from_secs(self.network_options.connection_timeout());

// Read buffered events from previous polls before calling a new poll

if let Some(event) = self.state.events.pop_front() {

return Ok(event);

}

let mut no_sleep = Box::pin(time::sleep(Duration::ZERO));

// this loop is necessary since self.incoming.pop_front() might return None. In that case,

// instead of returning a None event, we try again.

select! {

// Pull a bunch of packets from network, reply in bunch and yield the first item

o = network.readb(&mut self.state) => {

o?;

// flush all the acks and return first incoming packet

match time::timeout(network_timeout, network.flush()).await {

Ok(inner) => inner?,

Err(_)=> return Err(ConnectionError::FlushTimeout),

};

Ok(self.state.events.pop_front().unwrap())

},

// Handles pending and new requests.

// If available, prioritises pending requests from previous session.

// Else, pulls next request from user requests channel.

// If conditions in the below branch are for flow control.

// The branch is disabled if there's no pending messages and new user requests

// cannot be serviced due flow control.

// We read next user user request only when inflight messages are < configured inflight

// and there are no collisions while handling previous outgoing requests.

//

// Flow control is based on ack count. If inflight packet count in the buffer is

// less than max_inflight setting, next outgoing request will progress. For this

// to work correctly, broker should ack in sequence (a lot of brokers won't)

//

// E.g If max inflight = 5, user requests will be blocked when inflight queue

// looks like this                 -> [1, 2, 3, 4, 5].

// If broker acking 2 instead of 1 -> [1, x, 3, 4, 5].

// This pulls next user request. But because max packet id = max_inflight, next

// user request's packet id will roll to 1. This replaces existing packet id 1.

// Resulting in a collision

//

// Eventloop can stop receiving outgoing user requests when previous outgoing

// request collided. I.e collision state. Collision state will be cleared only

// when correct ack is received

// Full inflight queue will look like -> [1a, 2, 3, 4, 5].

// If 3 is acked instead of 1 first   -> [1a, 2, x, 4, 5].

// After collision with pkid 1        -> [1b ,2, x, 4, 5].

// 1a is saved to state and event loop is set to collision mode stopping new

// outgoing requests (along with 1b).

o = Self::next_request(

&mut self.pending,

&self.requests_rx,

self.mqtt_options.pending_throttle

), if !self.pending.is_empty() || (!inflight_full && !collision) => match o {

Ok(request) => {

if let Some(outgoing) = self.state.handle_outgoing_packet(request)? {

network.write(outgoing).await?;

}

match time::timeout(network_timeout, network.flush()).await {

Ok(inner) => inner?,

Err(_)=> return Err(ConnectionError::FlushTimeout),

};

Ok(self.state.events.pop_front().unwrap())

}

Err(_) => Err(ConnectionError::RequestsDone),

},

// We generate pings irrespective of network activity. This keeps the ping logic

// simple. We can change this behavior in future if necessary (to prevent extra pings)

_ = self.keepalive_timeout.as_mut().unwrap_or(&mut no_sleep),

if self.keepalive_timeout.is_some() && !self.mqtt_options.keep_alive.is_zero() => {

let timeout = self.keepalive_timeout.as_mut().unwrap();

timeout.as_mut().reset(Instant::now() + self.mqtt_options.keep_alive);

if let Some(outgoing) = self.state.handle_outgoing_packet(Request::PingReq(PingReq))? {

network.write(outgoing).await?;

}

match time::timeout(network_timeout, network.flush()).await {

Ok(inner) => inner?,

Err(_)=> return Err(ConnectionError::FlushTimeout),

};

Ok(self.state.events.pop_front().unwrap())

}

}

}

pub fn network_options(&self) -> NetworkOptions {

self.network_options.clone()

}

pub fn set_network_options(&mut self, network_options: NetworkOptions) -> &mut Self {

self.network_options = network_options;

self

}

async fn next_request(

pending: &mut VecDeque<Request>,

rx: &Receiver<Request>,

pending_throttle: Duration,

) -> Result<Request, ConnectionError> {

if !pending.is_empty() {

time::sleep(pending_throttle).await;

// We must call .pop_front() AFTER sleep() otherwise we would have

// advanced the iterator but the future might be canceled before return

Ok(pending.pop_front().unwrap())

} else {

match rx.recv_async().await {

Ok(r) => Ok(r),

Err(_) => Err(ConnectionError::RequestsDone),

}

}

}

}

/// This stream internally processes requests from the request stream provided to the eventloop

/// while also consuming byte stream from the network and yielding mqtt packets as the output of

/// the stream.

/// This function (for convenience) includes internal delays for users to perform internal sleeps

/// between re-connections so that cancel semantics can be used during this sleep

async fn connect(

mqtt_options: &MqttOptions,

network_options: NetworkOptions,

) -> Result<(Network, ConnAck), ConnectionError> {

// connect to the broker

let mut network = network_connect(mqtt_options, network_options).await?;

// make MQTT connection request (which internally awaits for ack)

let connack = mqtt_connect(mqtt_options, &mut network).await?;

Ok((network, connack))

}

pub(crate) async fn socket_connect(

host: String,

network_options: NetworkOptions,

) -> io::Result<TcpStream> {

let addrs = lookup_host(host).await?;

let mut last_err = None;

for addr in addrs {

let socket = match addr {

SocketAddr::V4(_) => TcpSocket::new_v4()?,

SocketAddr::V6(_) => TcpSocket::new_v6()?,

};

socket.set_nodelay(network_options.tcp_nodelay)?;

if let Some(send_buff_size) = network_options.tcp_send_buffer_size {

socket.set_send_buffer_size(send_buff_size).unwrap();

}

if let Some(recv_buffer_size) = network_options.tcp_recv_buffer_size {

socket.set_recv_buffer_size(recv_buffer_size).unwrap();

}

#[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "linux"))]

{

if let Some(bind_device) = &network_options.bind_device {

// call the bind_device function only if the bind_device network option is defined

// If binding device is None or an empty string it removes the binding,

// which is causing PermissionDenied errors in AWS environment (lambda function).

socket.bind_device(Some(bind_device.as_bytes()))?;

}

}

// Bind to a specific local address if configured.

// This enables distributing connections across multiple local IPs

// to avoid ephemeral port exhaustion when creating >50K connections.

if let Some(bind_addr) = network_options.bind_addr {

socket.bind(bind_addr)?;

}

match socket.connect(addr).await {

Ok(s) => return Ok(s),

Err(e) => {

last_err = Some(e);

}

};

}

Err(last_err.unwrap_or_else(|| {

io::Error::new(

io::ErrorKind::InvalidInput,

"could not resolve to any address",

)

}))

}

async fn network_connect(

options: &MqttOptions,

network_options: NetworkOptions,

) -> Result<Network, ConnectionError> {

// Process Unix files early, as proxy is not supported for them.

#[cfg(unix)]

if matches!(options.transport(), Transport::Unix) {

let file = options.broker_addr.as_str();

let socket = UnixStream::connect(Path::new(file)).await?;

let network = Network::new(

socket,

options.max_incoming_packet_size,

options.max_outgoing_packet_size,

);

return Ok(network);

}

// For websockets domain and port are taken directly from `broker_addr` (which is a url).

let (domain, port) = match options.transport() {

#[cfg(feature = "websocket")]

Transport::Ws => split_url(&options.broker_addr)?,

#[cfg(all(feature = "use-rustls-no-provider", feature = "websocket"))]

Transport::Wss(_) => split_url(&options.broker_addr)?,

_ => options.broker_address(),

};

let tcp_stream: Box<dyn AsyncReadWrite> = {

#[cfg(feature = "proxy")]

match options.proxy() {

Some(proxy) => proxy.connect(&domain, port, network_options).await?,

None => {

let addr = format!("{domain}:{port}");

let tcp = socket_connect(addr, network_options).await?;

Box::new(tcp)

}

}

#[cfg(not(feature = "proxy"))]

{

let addr = format!("{domain}:{port}");

let tcp = socket_connect(addr, network_options).await?;

Box::new(tcp)

}

};

let network = match options.transport() {

Transport::Tcp => Network::new(

tcp_stream,

options.max_incoming_packet_size,

options.max_outgoing_packet_size,

),

#[cfg(any(feature = "use-rustls-no-provider", feature = "use-native-tls"))]

Transport::Tls(tls_config) => {

let socket =

tls::tls_connect(&options.broker_addr, options.port, &tls_config, tcp_stream)

.await?;

Network::new(

socket,

options.max_incoming_packet_size,

options.max_outgoing_packet_size,

)

}

#[cfg(unix)]

Transport::Unix => unreachable!(),

#[cfg(feature = "websocket")]

Transport::Ws => {

let mut request = options.broker_addr.as_str().into_client_request()?;

request

.headers_mut()

.insert("Sec-WebSocket-Protocol", "mqtt".parse().unwrap());

if let Some(request_modifier) = options.request_modifier() {

request = request_modifier(request).await;

}

let (socket, response) =

async_tungstenite::tokio::client_async(request, tcp_stream).await?;

validate_response_headers(response)?;

Network::new(

WsStream::new(socket),

options.max_incoming_packet_size,

options.max_outgoing_packet_size,

)

}

#[cfg(all(feature = "use-rustls-no-provider", feature = "websocket"))]

Transport::Wss(tls_config) => {

let mut request = options.broker_addr.as_str().into_client_request()?;

request

.headers_mut()

.insert("Sec-WebSocket-Protocol", "mqtt".parse().unwrap());

if let Some(request_modifier) = options.request_modifier() {

request = request_modifier(request).await;

}

let connector = tls::rustls_connector(&tls_config).await?;

let (socket, response) = async_tungstenite::tokio::client_async_tls_with_connector(

request,

tcp_stream,

Some(connector),

)

.await?;

validate_response_headers(response)?;

Network::new(

WsStream::new(socket),

options.max_incoming_packet_size,

options.max_outgoing_packet_size,

)

}

};

Ok(network)

}

async fn mqtt_connect(

options: &MqttOptions,

network: &mut Network,

) -> Result<ConnAck, ConnectionError> {

let mut connect = Connect::new(options.client_id());

connect.keep_alive = options.keep_alive().as_secs() as u16;

connect.clean_session = options.clean_session();

connect.last_will = options.last_will();

connect.login = options.credentials();

// send mqtt connect packet

network.write(Packet::Connect(connect)).await?;

network.flush().await?;

// validate connack

match network.read().await? {

Incoming::ConnAck(connack) if connack.code == ConnectReturnCode::Success => Ok(connack),

Incoming::ConnAck(connack) => Err(ConnectionError::ConnectionRefused(connack.code)),

packet => Err(ConnectionError::NotConnAck(packet)),

}

}

You can’t perform that action at this time.