---
title: ipnstate package - tailscale.com/ipn/ipnstate - Go Packages
id: ipnstate-package-tailscalecomipnipnstate-go-packages
tags:
- linux-agent-jupiteros-fleet-15537b
- primary-source
- repo-source
- official-docs
- source-code
- repo-map
created: '2026-09-02T05:38:56.239774Z'
updated: '2026-09-02T17:37:22.182427Z'
source: https://pkg.go.dev/tailscale.com/ipn/ipnstate
source_domain: pkg.go.dev
fetched_at: '2026-09-02T05:38:48.315008Z'
fetch_provider: builtin
status: review
type: note
tier: institutional
content_type: docs
deprecated: false
summary: 'Authoritative Go package documentation (pkg.go.dev, tailscale.com/ipn/ipnstate,
  module v1.102.3 published Aug 2026) for the schema behind ''tailscale status --json''.
  This is the canonical reference for any agent consuming Tailscale status output.
  Key structures: Status (Version, TUN bool, BackendState string with documented values
  ''NoState'', ''NeedsLogin'', ''NeedsMachineAuth'', ''Stopped'', ''Starting'', ''Running'';
  TailscaleIPs []netip.Addr; Self *PeerStatus; ExitNodeStatus *ExitNodeStatus with
  ID/Online/TailscaleIPs; Health []string — ''Health contains health check problems.
  Empty means everything is good''; CurrentTailnet *TailnetStatus with MagicDNSSuffix/MagicDNSEnabled;
  Peer map[key.NodePublic]*PeerStatus; User map; ClientVersion). PeerStatus fields:
  ID, NodeID, HostName, DNSName (FQDN ending in dot), OS, TailscaleIPs, AllowedIPs,
  Tags, PrimaryRoutes, Addrs, CurAddr, Relay (DERP region), PeerRelay, RxBytes/TxBytes
  int64, Created/LastWrite/LastSeen/LastHandshake times, Online bool (connected to
  control plane), ExitNode bool (currently selected), ExitNodeOption bool (offered
  && approved), Active bool (''some packet sent to this peer in the past two minutes.
  That definition is subject to change''), InNetworkMap/InMagicSock/InEngine consistency
  booleans, Expired, KeyExpiry, Capabilities (deprecated -> CapMap), SSH_HostKeys,
  ShareeNode. Helper methods: PeerStatus.IsRouter() (added v1.102.0, ''exit node,
  subnet router, app connector''), IsTagged() (v1.52.0), HasCap() (v1.50.0). Also
  PingResult (tailscale ping response: IP, NodeIP, NodeName, Err, LatencySeconds,
  Endpoint, PeerRelay, DERPRegionID/Code), PeerStatusLite (NodeKey, TxBytes/RxBytes,
  LastHandshake), TailnetLockStatus/TKAPeer (tailnet lock signing state), UpdateProgress/SelfUpdateStatus
  (UpdateFinished/UpdateInProgress/UpdateFailed). WARNING in source: PeerStatus fields
  are merged by StatusBuilder.AddPeer — new fields must be handled in AddPeer merging
  or status data may be lost/inconsistent.'
---

*Suggested by [[ipnstate-package-tailscalecomipnipnstate-go-packages]] — fetch the latest-version ipnstate schema page to diff against the v1.102.3 page we already have*

ipnstate package - tailscale.com/ipn/ipnstate - Go Packages

ipnstate

package

Version:
v1.102.3

Opens a new window with list of versions in this module.

Latest

Latest

This package is not in the latest version of its module.

Go to latest

Published: Aug 19, 2026

License: BSD-3-Clause

Opens a new window with license information.

Imports: 15

Opens a new window with list of imports.

Imported by: 100

Opens a new window with list of known importers.

Main

Versions

Licenses

Imports

Imported By

Details

Valid go.mod file

The Go module system was introduced in Go 1.11 and is the official dependency management
solution for Go.

Redistributable license

Redistributable licenses place minimal restrictions on how software can be used,
modified, and redistributed.

Tagged version

Modules with tagged versions give importers more predictable builds.

Stable version

When a project reaches major version v1 it is considered stable.

Learn more about best practices

Repository

github.com/tailscale/tailscale

Links

Open Source Insights

Documentation
¶

Overview ¶

Package ipnstate captures the entire state of the Tailscale network.

It's a leaf package so ipn, wgengine, and magicsock can all depend on it.

Index ¶

func SortPeers(peers []*PeerStatus)

type DebugDERPRegionReport

type ExitNodeStatus

type NetworkLockStatusdeprecated

type NetworkLockUpdatedeprecated

type PeerStatus

func (ps *PeerStatus) HasCap(cap tailcfg.NodeCapability) bool

func (ps *PeerStatus) IsRouter() bool

func (ps *PeerStatus) IsTagged() bool

type PeerStatusLite

type PingResult

func (pr *PingResult) ToPingResponse(pingType tailcfg.PingType) *tailcfg.PingResponse

type SelfUpdateStatus

type Status

func (s *Status) Peers() []key.NodePublic

func (st *Status) WriteHTML(w io.Writer)

type StatusBuilder

func (sb *StatusBuilder) AddPeer(peer key.NodePublic, st *PeerStatus)

func (sb *StatusBuilder) AddTailscaleIP(ip netip.Addr)

func (sb *StatusBuilder) AddUser(id tailcfg.UserID, up tailcfg.UserProfileView)

func (sb *StatusBuilder) MutateSelfStatus(f func(*PeerStatus))

func (sb *StatusBuilder) MutateStatus(f func(*Status))

func (sb *StatusBuilder) Status() *Status

type StatusUpdater

type TKAKey

type TKAPeer

func (src *TKAPeer) Clone() *TKAPeer

type TaildropTargetStatus

type TailnetLockStatus

type TailnetLockUpdate

type TailnetStatus

type UpdateProgress

func NewUpdateProgress(ps SelfUpdateStatus, msg string) UpdateProgress

Constants ¶

This section is empty.

Variables ¶

This section is empty.

Functions ¶

func SortPeers ¶

added in
v1.4.0

func SortPeers(peers []*PeerStatus)

SortPeers sorts peers by either their DNS name, hostname, Tailscale IP,
or ultimately their current public key.

Types ¶

type DebugDERPRegionReport ¶

added in
v1.34.0

type DebugDERPRegionReport struct {
Info     []string
Warnings []string
Errors   []string
}

DebugDERPRegionReport is the result of a "tailscale debug derp" command,
to let people debug a custom DERP setup.

type ExitNodeStatus ¶

added in
v1.28.0

type ExitNodeStatus struct {
// ID is the exit node's ID.
ID tailcfg.StableNodeID

// Online is whether the exit node is alive.
Online bool

// TailscaleIPs are the exit node's IP addresses assigned to the node.
TailscaleIPs []netip.Prefix
}

ExitNodeStatus describes the current exit node.

type NetworkLockStatus
deprecated

added in
v1.30.0

type NetworkLockStatus = TailnetLockStatus

Deprecated: use TailnetLockStatus instead.

type NetworkLockUpdate
deprecated

added in
v1.34.0

type NetworkLockUpdate = TailnetLockUpdate

Deprecated: use TailnetLockUpdate instead.

type PeerStatus ¶

type PeerStatus struct {
ID        tailcfg.StableNodeID
NodeID    tailcfg.NodeID
PublicKey key.NodePublic
HostName  string // HostInfo's Hostname (not a DNS name or necessarily unique)

// DNSName is the Peer's FQDN. It ends with a dot.
// It has the form "host.<MagicDNSSuffix>."
DNSName string
OS      string // HostInfo.OS
UserID  tailcfg.UserID

// AltSharerUserID is the user who shared this node
// if it's different than UserID. Otherwise it's zero.
AltSharerUserID tailcfg.UserID `json:",omitempty"`

// TailscaleIPs are the IP addresses assigned to the node.
TailscaleIPs []netip.Addr
// AllowedIPs are IP addresses allowed to route to this node.
AllowedIPs *views.Slice[netip.Prefix] `json:",omitempty"`

// Tags are the list of ACL tags applied to this node.
// See tailscale.com/tailcfg#Node.Tags for more information.
Tags *views.Slice[string] `json:",omitempty"`

// PrimaryRoutes are the routes this node is currently the primary
// subnet router for, as determined by the control plane. It does
// not include the IPs in TailscaleIPs.
PrimaryRoutes *views.Slice[netip.Prefix] `json:",omitempty"`

// Endpoints:
Addrs     []string
CurAddr   string // one of Addrs, or unique if roaming
Relay     string // DERP region
PeerRelay string // peer relay address (ip:port:vni)

RxBytes        int64
TxBytes        int64
Created        time.Time // time registered with tailcontrol
LastWrite      time.Time // time last packet sent
LastSeen       time.Time // last seen to tailcontrol; only present if offline
LastHandshake  time.Time // with local wireguard
Online         bool      // whether node is connected to the control plane
ExitNode       bool      // true if this is the currently selected exit node.
ExitNodeOption bool      // true if this node can be an exit node (offered && approved)

// Active is whether the node was recently active. The
// definition is somewhat undefined but has historically and
// currently means that there was some packet sent to this
// peer in the past two minutes. That definition is subject to
// change.
Active bool

// PeerAPIURL are the URLs of the node's PeerAPI servers.
PeerAPIURL []string

// TaildropTargetStatus represents the node's eligibility to have files shared to it.
TaildropTarget TaildropTargetStatus

// Reason why this peer cannot receive files. Empty if CanReceiveFiles=true
NoFileSharingReason string

// Capabilities are capabilities that the node has.
// They're free-form strings, but should be in the form of URLs/URIs
// such as:
//    "https://tailscale.com/cap/is-admin"
//    "https://tailscale.com/cap/file-sharing"
//    "funnel"
//
// Deprecated: use CapMap instead. See https://github.com/tailscale/tailscale/issues/11508
// Every value is Capabilities is also a key in CapMap, even if it
// has no values in that map.
Capabilities []tailcfg.NodeCapability `json:",omitempty"`

// CapMap is a map of capabilities to their values.
CapMap tailcfg.NodeCapMap `json:",omitempty"`

// SSH_HostKeys are the node's SSH host keys, if known.
SSH_HostKeys []string `json:"sshHostKeys,omitempty"`

// ShareeNode indicates this node exists in the netmap because
// it's owned by a shared-to user and that node might connect
// to us. These nodes should be hidden by "tailscale status"
// etc by default.
ShareeNode bool `json:",omitempty"`

// InNetworkMap means that this peer was seen in our latest network map.
// In theory, all of InNetworkMap and InMagicSock and InEngine should all be true.
InNetworkMap bool

// InMagicSock means that this peer is being tracked by magicsock.
// In theory, all of InNetworkMap and InMagicSock and InEngine should all be true.
InMagicSock bool

// InEngine means that this peer is tracked by the wireguard engine.
// In theory, all of InNetworkMap and InMagicSock and InEngine should all be true.
InEngine bool

// Expired means that this peer's node key has expired, based on either
// information from control or optimisically set on the client if the
// expiration time has passed.
Expired bool `json:",omitempty"`

// KeyExpiry, if present, is the time at which the node key expired or
// will expire.
KeyExpiry *time.Time `json:",omitempty"`

Location *tailcfg.Location `json:",omitempty"`
}

PeerStatus describes a peer node and its current state.
WARNING: The fields in PeerStatus are merged by the AddPeer method in the StatusBuilder.
When adding a new field to PeerStatus, you must update AddPeer to handle merging
the new field. The AddPeer function is responsible for combining multiple updates
to the same peer, and any new field that is not merged properly may lead to
inconsistencies or lost data in the peer status.

func (*PeerStatus) HasCap ¶

added in
v1.50.0

func (ps *PeerStatus) HasCap(cap tailcfg.NodeCapability) bool

HasCap reports whether ps has the given capability.

func (*PeerStatus) IsRouter ¶

added in
v1.102.0

func (ps *PeerStatus) IsRouter() bool

IsRouter reports whether ps describes a router:
a node that routes addresses besides its own.
Examples: an exit node, a subnet router, an app connector, etc.
It is the analogue of tailcfg.Node.IsRouter.

func (*PeerStatus) IsTagged ¶

added in
v1.52.0

func (ps *PeerStatus) IsTagged() bool

IsTagged reports whether ps is tagged.

type PeerStatusLite ¶

added in
v1.6.0

type PeerStatusLite struct {
// NodeKey is this peer's public node key.
NodeKey key.NodePublic

// TxBytes/RxBytes are the total number of bytes transmitted to/received
// from this peer.
TxBytes, RxBytes int64

// LastHandshake is the last time a handshake succeeded with this peer. (Or
// we got key confirmation via the first data message, which is
// approximately the same thing.)
//
// The time.Time zero value means that no handshake has succeeded, at least
// since this peer was last known to WireGuard. (Tailscale removes peers
// from the wireguard peer that are idle.)
LastHandshake time.Time
}

type PingResult ¶

added in
v1.2.0

type PingResult struct {
IP       string // ping destination
NodeIP   string // Tailscale IP of node handling IP (different for subnet routers)
NodeName string // DNS name base or (possibly not unique) hostname

Err            string
LatencySeconds float64

// Endpoint is a string of the form "{ip}:{port}" if direct UDP was used. It
// is not currently set for TSMP.
Endpoint string

// PeerRelay is a string of the form "{ip}:{port}:vni:{vni}" if a peer
// relay was used. It is not currently set for TSMP. Note that this field
// is not omitted during JSON encoding if it contains a zero value. This is
// done for consistency with the Endpoint field; this structure is exposed
// externally via localAPI, so we want to maintain the existing convention.
PeerRelay string

// DERPRegionID is non-zero DERP region ID if DERP was used.
// It is not currently set for TSMP pings.
DERPRegionID int

// DERPRegionCode is the three-letter region code
// corresponding to DERPRegionID.
// It is not currently set for TSMP pings.
DERPRegionCode string

// PeerAPIPort is set by TSMP ping responses for peers that
// are running a peerapi server. This is the port they're
// running the server on.
PeerAPIPort uint16 `json:",omitempty"`

// PeerAPIURL is the URL that was hit for pings of type "peerapi" (tailcfg.PingPeerAPI).
// It's of the form "http://ip:port" (or [ip]:port for IPv6).
PeerAPIURL string `json:",omitempty"`

// IsLocalIP is whether the ping request error is due to it being
// a ping to the local node.
IsLocalIP bool `json:",omitempty"`
}

PingResult contains response information for the "tailscale ping" subcommand,
saying how Tailscale can reach a Tailscale IP or subnet-routed IP.
See tailcfg.PingResponse for a related response that is sent back to control
for remote diagnostic pings.

func (*PingResult) ToPingResponse ¶

added in
v1.24.0

func (pr *PingResult) ToPingResponse(pingType tailcfg.PingType) *tailcfg.PingResponse

type SelfUpdateStatus ¶

added in
v1.54.0

type SelfUpdateStatus string

const (
UpdateFinished   SelfUpdateStatus = "UpdateFinished"
UpdateInProgress SelfUpdateStatus = "UpdateInProgress"
UpdateFailed     SelfUpdateStatus = "UpdateFailed"
)

type Status ¶

type Status struct {
// Version is the daemon's long version (see version.Long).
Version string

// TUN is whether /dev/net/tun (or equivalent kernel interface) is being
// used. If false, it's running in userspace mode.
TUN bool

// BackendState is an ipn.State string value:
//  "NoState", "NeedsLogin", "NeedsMachineAuth", "Stopped",
//  "Starting", "Running".
BackendState string

// HaveNodeKey is whether the current profile has a node key configured.
HaveNodeKey bool `json:",omitempty"`

AuthURL      string       // current URL provided by control to authorize client
TailscaleIPs []netip.Addr // Tailscale IP(s) assigned to this node
Self         *PeerStatus

// ExitNodeStatus describes the current exit node.
// If nil, an exit node is not in use.
ExitNodeStatus *ExitNodeStatus `json:"ExitNodeStatus,omitempty"`

// Health contains health check problems.
// Empty means everything is good. (or at least that no known
// problems are detected)
Health []string

// This field is the legacy name of CurrentTailnet.MagicDNSSuffix.
//
// Deprecated: use CurrentTailnet.MagicDNSSuffix instead.
MagicDNSSuffix string

// CurrentTailnet is information about the tailnet that the node
// is currently connected to. When not connected, this field is nil.
CurrentTailnet *TailnetStatus

// CertDomains are the set of DNS names for which the control
// plane server will assist with provisioning TLS
// certificates. See SetDNSRequest for dns-01 ACME challenges
// for e.g. LetsEncrypt. These names are FQDNs without
// trailing periods, and without any "_acme-challenge." prefix.
CertDomains []string

// ExtraRecords contains extra DNS records to add to the DNS resolver.
ExtraRecords []tailcfg.DNSRecord

// Peer is the state of each peer, keyed by each peer's current public key.
Peer map[key.NodePublic]*PeerStatus

// User contains profile information about UserIDs referenced by
// PeerStatus.UserID, PeerStatus.AltSharerUserID, etc.
User map[tailcfg.UserID]tailcfg.UserProfile

// ClientVersion, when non-nil, contains information about the latest
// version of the Tailscale client that's available. Depending on
// the platform and client settings, it may not be available.
ClientVersion *tailcfg.ClientVersion
}

Status represents the entire state of the IPN network.

func (*Status) Peers ¶

func (s *Status) Peers() []key.NodePublic

func (*Status) WriteHTML ¶

func (st *Status) WriteHTML(w io.Writer)

type StatusBuilder ¶

type StatusBuilder struct {
WantPeers bool // whether caller wants peers
// contains filtered or unexported fields
}

StatusBuilder is a request to construct a Status. A new StatusBuilder is
passed to various subsystems which then call methods on it to populate state.
Call its Status method to return the final constructed Status.

func (*StatusBuilder) AddPeer ¶

func (sb *StatusBuilder) AddPeer(peer key.NodePublic, st *PeerStatus)

AddPeer adds a peer node to the status.

Its PeerStatus is mixed with any previous status already added.

func (*StatusBuilder) AddTailscaleIP ¶

added in
v1.0.0

func (sb *StatusBuilder) AddTailscaleIP(ip netip.Addr)

AddIP adds a Tailscale IP address to the status.

func (*StatusBuilder) AddUser ¶

func (sb *StatusBuilder) AddUser(id tailcfg.UserID, up tailcfg.UserProfileView)

AddUser adds a user profile to the status.

func (*StatusBuilder) MutateSelfStatus ¶

added in
v1.8.0

func (sb *StatusBuilder) MutateSelfStatus(f func(*PeerStatus))

MutateSelfStatus calls f with the PeerStatus of our own node to mutate.

It may not assume other fields of status are already populated, and
may not retain or write to the Status after f returns.

MutateStatus acquires a lock so f must not call back into sb.

func (*StatusBuilder) MutateStatus ¶

added in
v1.8.0

func (sb *StatusBuilder) MutateStatus(f func(*Status))

MutateStatus calls f with the status to mutate.

It may not assume other fields of status are already populated, and
may not retain or write to the Status after f returns.

func (*StatusBuilder) Status ¶

func (sb *StatusBuilder) Status() *Status

Status returns the status that has been built up so far from previous
calls to MutateStatus, MutateSelfStatus, AddPeer, etc.

type StatusUpdater ¶

type StatusUpdater interface {
UpdateStatus(*StatusBuilder)
}

type TKAKey ¶

added in
v1.34.0

type TKAKey struct {
Kind     string
Key      key.NLPublic
Metadata map[string]string
Votes    uint
}

TKAKey describes a key trusted by tailnet lock.

type TKAPeer ¶

added in
v1.74.0

type TKAPeer struct {
Name             string // DNS
ID               tailcfg.NodeID
StableID         tailcfg.StableNodeID
TailscaleIPs     []netip.Addr // Tailscale IP(s) assigned to this node
NodeKey          key.NodePublic
NodeKeySignature tka.NodeKeySignature
}

TKAPeer describes a peer and its tailnet lock details.

func (*TKAPeer) Clone ¶

added in
v1.74.0

func (src *TKAPeer) Clone() *TKAPeer

Clone makes a deep copy of TKAPeer.
The result aliases no memory with the original.

type TaildropTargetStatus ¶

added in
v1.82.0

type TaildropTargetStatus int

const (
TaildropTargetUnknown TaildropTargetStatus = iota
TaildropTargetAvailable
TaildropTargetNoNetmapAvailable
TaildropTargetIpnStateNotRunning
TaildropTargetMissingCap
TaildropTargetOffline
TaildropTargetNoPeerInfo
TaildropTargetUnsupportedOS
TaildropTargetNoPeerAPI
TaildropTargetOwnedByOtherUser
)

type TailnetLockStatus ¶

added in
v1.102.0

type TailnetLockStatus struct {
// Enabled is true if tailnet lock is enabled.
Enabled bool

// Head describes the AUM hash of the leaf AUM. Head is nil
// if tailnet lock is not enabled.
Head *[32]byte

// PublicKey describes the node's tailnet-lock public key.
// It may be zero if the node has not logged in.
PublicKey key.NLPublic

// NodeKey describes the node's current node-key. This field is not
// populated if the node is not operating (i.e. waiting for a login).
NodeKey *key.NodePublic

// NodeKeySigned is true if our node is authorized by tailnet-lock.
NodeKeySigned bool

// NodeKeySignature is the current signature of this node's key.
NodeKeySignature *tka.NodeKeySignature

// TrustedKeys describes the keys currently trusted to make changes
// to tailnet-lock.
TrustedKeys []TKAKey

// VisiblePeers describes peers which are visible in the netmap that
// have valid Tailnet Lock signatures.
VisiblePeers []*TKAPeer

// FilteredPeers describes peers which were removed from the netmap
// (i.e. no connectivity) because they failed tailnet lock
// checks.
FilteredPeers []*TKAPeer

// StateID is a nonce associated with the tailnet lock authority,
// generated upon enablement. This field is not populated if the
// tailnet lock is disabled.
StateID uint64
}

TailnetLockStatus represents whether tailnet-lock is enabled,
along with details about the locally-known state of the tailnet
key authority.

type TailnetLockUpdate ¶

added in
v1.102.0

type TailnetLockUpdate struct {
Hash   [32]byte
Change string // values of tka.AUMKind.String()

// Raw contains the serialized AUM. The AUM is sent in serialized
// form to avoid transitive dependences bloating this package.
Raw []byte
}

TailnetLockUpdate describes a change to tailnet-lock state.

type TailnetStatus ¶

added in
v1.22.0

type TailnetStatus struct {
// Name is the name of the network that's currently in use.
Name string

// MagicDNSSuffix is the network's MagicDNS suffix for nodes
// in the network such as "userfoo.tailscale.net".
// There are no surrounding dots.
// MagicDNSSuffix should be populated regardless of whether a domain
// has MagicDNS enabled.
MagicDNSSuffix string

// MagicDNSEnabled is whether or not the network has MagicDNS enabled.
// Note that the current device may still not support MagicDNS if
// `--accept-dns=false` was used.
MagicDNSEnabled bool
}

TailnetStatus is information about a Tailscale network ("tailnet").

type UpdateProgress ¶

added in
v1.54.0

type UpdateProgress struct {
Status  SelfUpdateStatus `json:"status,omitempty"`
Message string           `json:"message,omitempty"`
Version string           `json:"version,omitempty"`
}

func NewUpdateProgress ¶

added in
v1.54.0

func NewUpdateProgress(ps SelfUpdateStatus, msg string) UpdateProgress

Source Files
¶

View all Source files

ipnstate.go
ipnstate_clone.go

Click to show internal directories.

Click to hide internal directories.

Jump to

Close

Keyboard shortcuts

? : This menu

/ : Search site

f or F : Jump to

y or Y
: Canonical URL

Close

go.dev uses cookies from Google to deliver and enhance the quality of its services and to
analyze traffic. Learn more.

Okay