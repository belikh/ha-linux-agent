---
title: MQTT Client Component - ESPHome - Smart Home Made Simple
id: mqtt-client-component-esphome-smart-home-made-simple
tags:
- linux-agent-jupiteros-fleet-15537b
- mqtt
- home-assistant
- mqtt-discovery
- availability
- source-code
- official-docs
- primary-source
- birth-message
created: '2026-09-02T06:41:31.061743Z'
updated: '2026-09-05T10:51:22.035409Z'
source: https://esphome.io/components/mqtt/
source_domain: esphome.io
fetched_at: '2026-09-02T06:41:26.501385Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: 'ESPHome''s MQTT client component doc — the canonical reference implementation
  of birth/LWT/availability semantics that a Linux fleet agent should mirror (ESPHome
  is HA''s own embedded ecosystem, so its patterns are effectively HA-endorsed). Key
  contract: retained birth message to <TOPIC_PREFIX>/status payload ''online'' on
  connect; broker-side LWT to the same topic with payload ''offline'' on ungraceful
  drop — ''If the node is not connected to MQTT, Home Assistant will show all its
  entities as unavailable''. Availability is DISABLED if birth and will topics differ
  or are empty. Other load-bearing config: shutdown_message (clean-shutdown variant
  LWT can''t cover), reboot_timeout (reboot node after 15min without MQTT connection
  — a self-healing pattern directly transplantable to a systemd agent), keepalive
  15s default, on_connect/on_disconnect automations, wait_for_connection to block
  component startup until connected, publish_nan_as_none (publish None instead of
  NaN for HA Unknown/Unavailable handling), idf_send_async for non-blocking publish
  under poor networks, and object_id generator discovery_object_id_generator: device_name
  to disambiguate entity names across a fleet (HA 2021.12+). Documents the retained-message
  staleness problem: stale retained discovery messages reappear on every HA restart;
  esphome clean-mqtt purges <DISCOVERY_PREFIX>/+/NODE_NAME/#. Also documents mqtt.discover_ip
  device-level discovery (esphome/discover topic, JSON payload with ip/mac/version)
  as distinct from entity discovery. TLS via certificate_authority with the MbedTLS
  wildcard-CN caveat. HA 2026.9-current doc set.'
---

MQTT Client Component - ESPHome - Smart Home Made SimpleSkip to content

SearchCtrlK
Cancel

Select themeDarkLightAutoGet started

Getting Started
Install ESPHome
Getting Started
Running in Docker
Ready-Made Projects
Migrate from Tasmota
FAQ and Tips
ESPHome Starter Kit
Components
All Components
1-Wire Bus
1-Wire Bus
1-Wire Bus via DS2484
DS248x 1-Wire Bus Master
1-Wire Bus via GPIO
AirTouch AT581x Radar
Alarm Control Panel Component
Alarm Control Panel Component
Template Alarm Control Panel
Atlas Scientific Peristaltic Pump
Audio ADC Core
Audio ADC Core
ES7210
ES7243E
Audio Codec Configuration
Audio DAC Core
Audio DAC Core
AIC3204
ES8156
ES8311
ES8388
PCM5122
Audio File Component
Audio File Component
Binary Sensor Component
Binary Sensor Component
Analog Threshold Binary Sensor
Bluetooth Low Energy Device
CAP1188 Capacitive Touch Sensor
ESP32 Touch Pad
GPIO Binary Sensor
Haier Climate Binary Sensors
Home Assistant Binary Sensor
Hydreon Rain Sensor Binary Sensor
LVGL Binary Sensor
Modbus Controller Binary Sensor
MPR121 Capacitive Touch Sensor
Nextion Binary Sensor Component
NFC Binary Sensor
Packet Transport Binary Sensor
PN532 NFC/RFID
Qwiic PIR Motion Binary Sensor
RC522 NFC/RFID
RDM6300 NFC/RFID
SDL Binary Sensor
Status Binary Sensor
Switch Binary Sensor
Template Binary Sensor
TTP229 Capacitive Touch Sensor
Tuya Binary Sensor
BK72xx Bluetooth Low Energy
BK72xx Bluetooth Low Energy Tracker Hub
BLE Client
BLE Component
BLE Server
Bluetooth Proxy
Button Component
Button Component
Factory Reset Button
Haier Climate Buttons
Generic Output Button
Restart Button
Safe Mode Button
Shutdown Button
Template Button
UART Button
Wake-on-LAN Button
Camera Component
Camera Component
Camera Encoder
CAN Bus
CAN Bus
ESP32 CAN
MCP2515
Captive Portal
CC1101 Low-Power Sub-1 GHz RF Transceiver
CH422G I/O Expander
CH423 I/O Expander
Climate Component
Climate Component
Anova Cooker
Bang Bang Climate Controller
BedJet
IR Remote Climate
Haier Climate
Midea Air Conditioner
Mitsubishi CN105 Climate
PID Climate
Thermostat Climate Controller
Tuya Climate
Copy Component
Cover Component
Cover Component
AM43 Cover
Current Based Cover
Endstop Cover
Feedback Cover
HE60R Garage Door Opener
Hörmann Cover
Template Cover
Time Based Cover
Tormatic/Novoferm Cover
Tuya Cover
Datetime Component
Datetime Component
Template Datetime
Debug Component
Deep Sleep Component
Demo Component
DF-Player mini
DFRobot mmWave Radar
Display Component
Display Component
Addressable Light
ePaper SPI Display
HUB75 RGB LED Matrix Display
ILI9xxx TFT LCD Series
Inkplate 5, 6, 10 and 6 Plus
IT8951 E-Paper Display
Character-Based LCD Display
MAX7219 7-Segment Display
MAX7219 Digit Display
MIPI DSI Display Driver
MIPI RGB Display Driver
MIPI SPI Display Driver
Nextion TFT LCD Display
PCD 8544 Display (Nokia 5110/3310)
Divoom Pixoo Display
PVVX MiThermometer Display
Quad SPI Displays
RPI_DPI_RGB Display Driver
SDL2 Display on host platform
SSD1306 OLED Display
SSD1322 OLED Display
SSD1325/7 OLED Display
SSD1327 OLED Display
SSD1331 OLED Display
SSD1351 OLED Display
ST7567 LCD Graphic Display
ST7701S Display Driver
ST7735 Display
ST7789V TFT LCD
ST7920 LCD Graphic Display
TM1621 LCD Display
TM1637 7-Segment Display
TM1638 7 Segment Display Keypad & LED Module
Waveshare E-Paper Display
Display Menu
Display Menu
Graphical Display Menu
LCD Menu
EMC2101 Fan Controller and Temperature sensor
ESP32 Bluetooth Low Energy Beacon
ESP32 Bluetooth Low Energy Tracker Hub
ESP32 Camera Component
ESP32 Camera Web Server Component
ESP32 Hosted
ESP32 Platform
ESP32-P4 LDO
ESP8266 Platform
ESPHome Core Configuration
ESPNow communication Component
Ethernet Component
Event Component
Event Component
Template Event
UART Event
Exposure Notification Listener
External Components
Factory Reset
Fan Component
Fan Component
Binary Fan
H-bridge Fan
Speed Fan
Template Fan
Tuya Fan
Font Renderer Component
Generic I²C device component
Global Variables
GPS Component
Graph Component
Grove TB6612FNG Motor Drive
Grow Fingerprint Reader
HLK-FM22x Face Recognition Module
Host Platform
HTTP Request
I²C Bus
I²S Audio Component
Images
Images
Animation
File
Online Image Component
Sendspin Image
Improv via BLE
Improv via Serial
Infrared Component
Infrared Component
Interval Component
IR/RF Proxy
json Component
Key collector component
LibreTiny Platform
Light Component
Light Component
Beken SPI LED Strip
Binary Light
Color Temperature Light
Cold White + Warm White Light
ESP32 RMT LED Strip
FastLED Light
H-bridge Light
LVGL Light
Monochromatic Light
NeoPixelBus Light
Light Partition
Pixoo Brightness
RGB Light
RGBCT Light
RGBW Light
RGBWW Light
RP2040 PIO LED Strip
Shelly Dimmer
Sonoff D1 Dimmer
SPI LED Strip Light
Status LED Light
Tuya Dimmer
LightWaveRF
LN882H Bluetooth Low Energy
LN882H Bluetooth Low Energy Tracker Hub
Lock Component
Lock Component
Generic Output Lock
Template Lock
Logger Component
LVGL Graphics
LVGL Graphics
LVGL Animations
LVGL Layouts
LVGL Widgets
Mapping Component
Matrix keypad
MAX6956 I/O Expander
MCP230xx I/O Expander
MCP23Sxx I/O Expander
mDNS Component
Media Player Components
Media Player Components
Sendspin Group Media Player
Speaker Audio Media Player
Speaker Source Media Player
Media Source Components
Media Source Components
Audio File Media Source
Audio HTTP Media Source
Sendspin Media Source
Micro Wake Word
MicroNova based pellet stove
Microphone Components
Microphone Components
I²S Audio Microphone
Modbus Client
Modbus Component
Modbus Controller
Modbus Server
Motion Component (IMU)
Motion Component (IMU)
BMI270 Accelerometer/Gyroscope Sensor
LSM6DS Accelerometer/Gyroscope Sensor
QMI8658 Accelerometer/Gyroscope Sensor
MQTT Client Component
Native API Component
Network component
Nordic UART Service (NUS)
NRF52 Platform
Number Component
Number Component
Home Assistant Number
LVGL Number
Modbus Controller Number
Template Number
Tuya Number
OpenTherm
OpenThread Component
Output Component
Output Component
AC Dimmer Component
BLE Client Binary Output
BP1658CJ LED driver
BP5758D LED driver
DAC7678
ESP32 DAC
ESP8266 Software PWM Output
GP8403 Component
GPIO Output
ESP32 LEDC Output
LibreTiny PWM Output
MCP4461 Component
MCP4725 Output
MCP4728 Component
MCP47A1 Output
Modbus Controller Output
MY9231/MY9291 LED driver
PCA9685
Sigma-Delta Output
Slow PWM Output
SM16716 LED driver
SM2135 LED driver
SM2235 LED driver
SM2335 LED driver
Template Output
TLC59208F
TLC5947 LED driver
TLC5971 LED driver
X9C Potentiometer Output
Zephyr PWM Output
Over-the-Air Updates
Over-the-Air Updates
ESPHome OTA Updates
OTA Update via HTTP Request
Web Server OTA Updates
nRF52 Firmware Updates
Packages
Packet Transport Component
Packet Transport Component
ESP-NOW Packet Transport Platform
SX126x Packet Transport Platform
SX127x Packet Transport Platform
UART Packet Transport Platform
UDP Packet Transport Platform
PCA6416A I/O Expander
PCA9554 I/O Expander
PCF8574 I/O Expander
PI4IOE5V6408 8-Bit I2C I/O Expander
PipSolar PV Inverter
PN7150 NFC
PN7160 NFC
Power Supply Component
Prometheus Component
Provisioning
PSRAM
Pylontech Battery
QR Code Component
Radio Frequency Component
Radio Frequency Component
Remote Receiver
Remote Transmitter
RF Bridge Component
RP2 BLE Tracker
RP2 Platform
RP2040 BLE Component
RTTTL Buzzer
Runtime Statistics
Safe Mode
Script Component
Seeed Studio MR24HPC1 mmWave (Kit)
Seeed Studio MR60BHA2 60GHz mmWave Breathing and Heartbeat Detection Sensor Kit
Seeed Studio MR60FDA2 60GHz mmWave Fall Detection Sensor Kit
Select Component
Select Component
Logger Select
LVGL Select
Modbus Controller Select
Template Select
Tuya Select
Sendspin
Sensor Component
Sensor Component
A01NYUB Waterproof Ultrasonic Sensor
A02YYUW Waterproof Ultrasonic Sensor
Absolute Humidity
Analog To Digital Sensor
ADC128S102 8-Channel 12-Bit A/D Converter
ADE7880 Power Sensor
ADE7953 Power Sensor
ADS1115 4-Channel 16-Bit A/D Converter
ADS1118 4-Channel 16-Bit A/D Converter with Internal Temperature Sensor
AGS10 Volatile Organic Compound (VOC) Sensor
AHT10 Temperature+Humidity Sensor
AirThings BLE Sensors
Grundfos Alpha3
AM2315C Temperature+Humidity Sensor
AM2320 Temperature+Humidity Sensor
AM43 Sensor
APDS9306 Sensor
APDS9960 Sensor
Air Quality Index (AQI)
AMS AS3935 Franklin Lightning Sensor
AS5600 12-Bit Magnetic Position Sensor
AS7341 Spectral Color Sensor
ATC MiThermometer
ATM90E26 Power Sensor
ATM90E32 Power Sensor
b-parasite
BH1750 Ambient Light Sensor
BH1900NUX Temperature Sensor
Binary Sensor Map
Belling BL0906 Energy Monitor
BL0939 Power Sensor
Belling BL0940 Energy Monitor
Belling BL0942 Energy Monitor
BLE Client Sensor
Bluetooth Low Energy RSSI Sensor
BME280 Temperature+Pressure+Humidity Sensor
BME680 Temperature+Pressure+Humidity+Gas Sensor
BME680 Temperature+Pressure+Humidity+Gas Sensor via BSEC
BME68x Temperature, Humidity, Pressure & Gas Sensor via BSEC2
BMI160 Accelerometer/Gyroscope Sensor
BMI270 Temperature Sensor
BMP085 Temperature+Pressure Sensor
BMP280 Temperature+Pressure Sensor
BMP388 / BMP390 Temperature+Pressure Sensor
BMP581 Temperature+Pressure Sensor
BTHome MiThermometer Sensor
CCS811 eCO_2 and Volatile Organic Compound Sensor
cd74hc4067 Analog Multiplexer
CUBIC CM1106 Single Beam NDIR CO2 Sensor Module
Combine the state of several sensors
CS5460A Power Sensor
CSE7761 Power Sensor
CSE7766 Power Sensor
CT Clamp Current Sensor
Dallas Temperature Sensor
Daly BMS Sensor
Dew Point
DHT Temperature+Humidity Sensor
DHT12 Temperature+Humidity Sensor
DLMS Meter
DPS310/DPS368 Atmospheric Pressure Sensor
DSMR Component
Duty Cycle Sensor
Duty Time
EE895 CO₂, Temperature and Pressure Sensor
OpenEnergyMonitor EmonTx Sensors
ENS160 Volatile Organic Compound (VOC) and eCO₂ Sensor
ENS210 Temperature+Humidity Sensor
EZO sensor circuits
FS3000 Air Velocity Sensor
Panasonic SN-GCJA5 Particulate Matter Sensor
FTLab GDK101 Gamma Radiation Sensor Module
GL-R01 I²C - Time Of Flight Mini LiDAR Laser Ranging Sensor
Sharp GP2Y1010AU0F PM2.5 Sensor
Grove Multichannel Gas Sensor V2
Growatt Solar
Haier Climate Sensors
Havells Solar
HC8 CO₂ Sensor
HDC1080 Temperature+Humidity Sensor
HDC2010 High Precision Temperature and Humidity Sensor
HDC2080 Temperature+Humidity Sensor
HDC302x Temperature and Humidity Sensor
HLW8012 Power Sensor
HLW8032 Power Sensor
The Grove - Laser PM2.5 Sensor (HM3301)
HMC5883L Magnetometer
Home Assistant Sensor
Honeywell HumidIcon (I2C HIH series) Temperature & Humidity Sensor
Honeywell ABP Pressure Sensors
Honeywell ABP 2 Pressure Sensors
HRXL/XL MaxSonar WR Series
HTE501 Temperature+Humidity Sensor
HTU21D | Si7021 | SHT21 Temperature & Humidity Sensor
HTU31D Temperature & Humidity Sensor
HX711 Load Cell Amplifier
Hydreon Rain Sensor
HYT271 Temperature & Humidity Sensor
AMS iAQ-Core Indoor Air Quality Sensor
INA219 DC Current Sensor
INA226 DC current and power sensor
INA260 DC Current and Power sensor
INA2xx family of digital power monitors
INA3221 3-Channel DC Current Sensor
Inkbird IBS-TH1, IBS-TH1 Mini, and IBS-TH2 BLE Sensor
Integration Sensor
Internal Temperature Sensor
JSN-SR04T Waterproof Ultrasonic Range Finder
Kamstrup Meter Protocol [KMP]
M5Stack KMeterISO I2C K-Type probe temperature sensor
Kuntze pool monitor
LC709203F Battery Monitor
LD2410 Sensor
LD2412 Sensor
LD2420 24GHz mmWave Radar Sensor
LD2450 Sensor
LD6002B 3D Presence Radar
LM75B Temperature Sensor
LPS22 Barometric Pressure Sensor
LSM6DS Temperature Sensor
LTR390 UV and Ambient Light Sensor
Lite-On Ambient Light & Proximity Sensors
Lite-On Ambient Light & Proximity Sensors
LVGL Sensor
M5Stack Unit 8 Angle
Analog Devices MAX17043 battery fuel gauge
MAX31855 K-Type Thermocouple Temperature Sensor
MAX31856 Thermocouple Temperature Sensor
MAX31865 Platinum RTD Temperature Sensor
MAX44009 Ambient Light Sensor
MAX6675 K-Type Thermocouple Temperature Sensor
MAX9611/9612 High Side Current+Voltage+Temperature Sensor
MCP3008 8-Channel 10-Bit A/D Converter
MCP3204 & MCP3208 12-Bit A/D Converters
MCP3221 12-bit ADC
MCP9600 Thermocouple Amplifier
MCP9808 Temperature Sensor
MH-Z19 CO_2 and Temperature Sensor
MiCS 4514 Gas Sensor
MLX90393 Triple-axis Magnetometer
MLX90614  non-contact thermometer
MMC5603 Magnetometer
MMC5983 Magnetometer
Modbus Controller Sensor
Mopeka Pro Check BLE Sensor
Mopeka Standard Check BLE Sensor
Motion Sensor
MPL3115A2 Barometric Pressure/Altitude/Temperature Sensor
MPU6050 Accelerometer/Gyroscope Sensor
MPU6886 Accelerometer/Gyroscope Sensor
MQTT Subscribe Sensor
MS5611 Atmospheric Pressure Sensor
MS8607 Temperature+Pressure+Humidity Sensor
MSA301 and MSA311 Sensors
NAU7802 24-bit ADC
Nextion Sensor Component
NPI-19 Pressure Sensor
NTC Sensor
Number Sensor
OpenThread Info Sensor
OPT3001 Ambient Light Sensor
Packet Transport Sensor
PM1006 Particulate Matter Sensor
CUBIC PM2005/PM2105 Laser Particle Sensor Module
PMSA003I Particulate Matter Sensor
PMSX003 Particulate Matter Sensor
PMWCS3 Capacitive Soil Moisture and Temperature Sensor
Pulse Counter Sensor
Pulse Meter Sensor
Pulse Width Sensor
Peacefair PZEM-004T Energy Monitor
Peacefair PZEM-004T V3 Energy Monitor
Peacefair PZEM-00X DC Energy Monitor
QMC5883L Magnetometer
QMI8658 Temperature Sensor
QMP6988 Temperature+Pressure Sensor
Radon Eye BLE Sensors
RD-03D mmWave Radar
Resistance Sensor
Rotary Encoder Sensor
RuuviTag Open Source BLE Sensor
SCD30 CO₂, Temperature and Relative Humidity Sensor
SCD4X CO₂, Temperature and Relative Humidity Sensor
Eastron SDM Energy Monitor
SDP3x / SDP800 Series Differential Pressure Sensor
SDS011 Particulate Matter Sensor
Selec Energy Monitor
SEN0321 DFRobot Ozone Sensor
SEN21231 Person Sensor from Useful Sensors
Sen5x Series Environmental sensor
SEN6x Series Environmental Sensor
Sendspin Sensor
SenseAir CO_2 Sensor
SFA30 Formaldehyde Sensor
SGP30 CO₂ and Volatile Organic Compound Sensor
SGP40 Volatile Organic Compound Sensor and SGP41 VOC and NOx Sensor
SHT3X-D Temperature+Humidity Sensor
SHT4X Temperature and Humidity Sensor
SHTCx Temperature+Humidity Sensors
SM300D2 7-in-1 Air Quality Sensor
SMT100 Soil Moisture Sensor
Sound Level Sensor
SPA06-003 Temperature+Pressure Sensor
SPS30 Particulate Matter Sensor
STS3X Temperature Sensor
STTS22H Temperature Sensor
SY6970 Battery Management IC
T6613/15 CO2 Sensors
TC74 Temperature Sensor
TCS34725 RGB Color Sensor
TEE501 Temperature Sensor
Teleinformation from Linky electrical counter.
TE-M3200 Pressure Sensor
Template Sensor
ThermoPro BLE Sensors
TMP102 Temperature Sensor
TMP1075 Temperature Sensor
TMP117 Temperature Sensor
TOF10120 Time Of Flight Distance Sensor
Total Daily Energy Sensor
TSL2561 Ambient Light Sensor
TSL2591 Ambient Light Sensor
Tuya Sensor
TX20/TX23 Wind Speed/Direction Sensor
uFire Isolated EC sensor
uFire ISE pH sensor
Ultrasonic Distance Sensor
Uptime Sensor
VEML3235 Ambient Light Sensor
VEML7700 and VEML6030 Ambient Light Sensors
VL53L0X Time Of Flight Distance Sensor
WiFi Signal Sensor
WTS01 Temperature Sensor
XIDIBEI XDB401 Pressure Sensor
CFSensor XGZP68xx Non-C Series Differential Pressure Sensor
Xiaomi Mijia BLE Sensors
HHCCJCY01 Moved To Xiaomi BLE
HHCCJCY10 Xiaomi MiFlora (Pink version)
LYWSDCGQ Moved To Xiaomi BLE
Xiaomi Miscale Sensors
Xiaomi Miscale2 combined into Xiaomi Miscale
Zio Ultrasonic Distance Sensor
ZyAura CO2 & Temperature & Humidity Sensor
Serial Proxy
Servo Component
Sim800L Component
SML (Smart Message Language)
SN74HC165 I/O Expander (shift register)
SN74HC595 I/O Expander (shift register)
Speaker Components
Speaker Components
I²S Audio Speaker
Mixer Speaker
Resampler Speaker
Router Speaker
SPI Bus
Sprinkler Controller
StatsD
Status LED
Stepper Component
Stepper Component
Substitutions
Sun
SUN_GTIL2 Grid Tie Inverter
Switch Component
Switch Component
BLE Client Switch
Factory Reset Switch
GPIO Switch
Haier Climate Switches
H-bridge Switch
Home Assistant Switch
LVGL Switch
Modbus Controller Switch
Nextion Switch Component
Generic Output Switch
Restart Switch
Safe Mode Switch
Shutdown Switch
Template Switch
Tuya Switch
UART Switch
SX126x Component
SX127x Component
SX1509 16 channel I/O Expander with LED driver and keypad engine
Syslog Component
TCA9548A I²C Multiplexer
TCA9555 I/O Expander
Text Component
Text Component
LVGL Text
Template Text
Text Sensor Component
Text Sensor Component
BLE Client Text Sensor
Bluetooth Low Energy Scanner
Ethernet Info Text Sensor
Haier Climate Text Sensors
Home Assistant Text Sensor
Key Collector Text Sensor
LibreTiny Text Sensor
LVGL Text Sensor
Modbus Controller Text Sensor
MQTT Subscribe Text Sensor
Nextion Text Sensor Component
OpenThread Info Text Sensor
Sendspin Text Sensor
Template Text Sensor
Text Text Sensor
Tuya Text Sensor
Uptime Text Sensor
Version Text Sensor
WiFi Info Text Sensor
WL-134 Pet Tag Sensor
Time Component
Time Component
BM8563 Time Source
DS1307 Time Source
GPS Time Source
Home Assistant Time Source
Host Time Source
PCF85063 Time Source
PCF8563 Time Source
RX8130 Time Source
SNTP Time Source
Zigbee Time Source
TinyUSB
TM1651 Battery Display
Touchscreen Components
Touchscreen Components
AXS15231 Touch Screen Controller
chsc6x Touch Screen Controller
cst226 Touch Screen Controller
CST328 Touch Screen Controller
cst816 Touch Screen Controller
cst9220 Touch Screen Controller
EKTF2232 Touchscreen Controller
FT5X06 Touch Screen Controller
FT63X6 Touchscreen Controller
GSL3670 Touch Screen Controller
gt911 Touch Screen Controller
Lilygo T5 4.7\
SDL2 Touch Screen Emulator
st7123 Touch Screen Controller
TT21100 Touch Screen Controller
XPT2046 Touch Screen Controller (Updated version)
Tuya MCU
UART Bus
UDP Component
UFM-01 Flow Meter
Update Core
Update Core
ESP32 Hosted Co-processor Update
Managed Updates via HTTP Request
Uponor Smatrix Base Pulse Underfloor Heating
USB CDC-ACM Interface
USB Host Interface
USB Host UART Interface
Valve Component
Valve Component
Template Valve
VBus Component
Voice Assistant
Water Heater Component
Water Heater Component
Template Water Heater
Waveshare CH32V003 I/O Expander
Web Server Component
WeiKai SPI/I²C UART/IO Expander
Wiegand keypad and tag reader
WiFi Component
WireGuard Component
XL9535 I/O Expander
XXTEA Component
Z-Wave Proxy
Zephyr BLE Server
Zigbee Component
Automations
Automation
Actions, Triggers, Conditions
Templates
Guides
Guides
Create audio clip files for use with I²S Speakers
Command Line Interface
Configuration Types
Sharing ESPHome devices
Using an ESP devboard as a USB-UART bridge
DIY Examples
ESP-IDF toolchain
ESP32 Arduino to ESP-IDF Migration Guide
Frequently Asked Questions
Made for ESPHome
Migrating from ESPEasy
Migrating from ESPurna
Migrating from Tasmota
Physically Connecting to your Device
Security Best Practices
Setting up RMT Devices
Contributors
Troubleshooting
Understanding Boards in ESPHome
YAML Configuration in ESPHome
Cookbook
Cookbook
BME280 Environment
Time & Temperature on OLED Display
ehmtx a matrix status display
Simple Garage Door
Infostripe
Lambda Magic
ESP32 Water Leak Detector (with notification)
LVGL: Tips and Tricks
Remote sensor with Mitsubishi CN105 heatpump
Non-Invasive Power Meter
Pulse Catcher
Sonoff Fish Pond Pump
Keeping Up
Blog
Changelog
Discord
Forums
Development
Supporters
Changelog
ESPHome 2026.8.0 - August 2026
ESPHome 2026.7.0 - July 2026
ESPHome 2026.6.0 - June 2026
ESPHome 2026.5.0 - May 2026
ESPHome 2026.4.0 - April 2026
ESPHome 2026.3.0 - March 2026
ESPHome 2026.2.0 - February 2026
ESPHome 2026.1.0 - January 2026
ESPHome 2025.12.0 - December 2025
ESPHome 2025.11.0 - November 2025
ESPHome 2025.10.0 - 15th October 2025
ESPHome 2025.9.0 - 17th September 2025
ESPHome 2025.8.0 - 20th August 2025
ESPHome 2025.7.0 - 16th July 2025
ESPHome 2025.6.0 - 18th June 2025
ESPHome 2025.5.0 - 21st May 2025
ESPHome 2025.4.0 - 16th April 2025
ESPHome 2025.3.0 - 19th March 2025
ESPHome 2025.2.0 - 19th February 2025
ESPHome 2024.12.0 - 18th December 2024
ESPHome 2024.11.0 - 20th November 2024
ESPHome 2024.10.0 - 16th October 2024
ESPHome 2024.9.0 - 18th September 2024
ESPHome 2024.8.0 - 21st August 2024
ESPHome 2024.7.0 - 17th July 2024
ESPHome 2024.6.0 - 19th June 2024
ESPHome 2024.5.0 - 15th May 2024
ESPHome 2024.4.0 - 17th April 2024
ESPHome 2024.3.0 - 20th March 2024
ESPHome 2024.2.0 - 21st February 2024
ESPHome 2023.12.0 - 20th December 2023
ESPHome 2023.11.0 - 15th November 2023
ESPHome 2023.10.0 - 18th October 2023
ESPHome 2023.9.0 - 27th September 2023
ESPHome 2023.8.0 - 16th August 2023
ESPHome 2023.7.0 - 19th July 2023
ESPHome 2023.6.0 - 21st June 2023
ESPHome 2023.5.0 - 17th May 2023
ESPHome 2023.4.0 - 19th April 2023
ESPHome 2023.3.0 - 15th March 2023
ESPHome 2023.2.0 - 15th February 2023
ESPHome 2022.12.0 - 14th December 2022
ESPHome 2022.11.0 - 16th November 2022
ESPHome 2022.10.0 - 19th October 2022
ESPHome 2022.9.0 - 21st September 2022
ESPHome 2022.8.0 - 17th August 2022
ESPHome 2022.6.0 - 15th June 2022
ESPHome 2022.5.0 - 18th May 2022
ESPHome 2022.4.0 - 20th April 2022
ESPHome 2022.3.0 - 16th March 2022
ESPHome 2022.2.0 - 16th February 2022
ESPHome 2022.1.0 - 19th January 2022
ESPHome 2021.12.0 - 11th December 2021
ESPHome 2021.11.0 - 17th November 2021
ESPHome 2021.10.0 - 20th October 2021
ESPHome 2021.9.0 - 15th September 2021
ESPHome 2021.8.0 - 18th August 2021
Changelog - Version 1.20.0 - 21st July 2021
Changelog - Version 1.19.0 - 16th June 2021
Changelog - Version 1.18.0 - 19th May 2021
Changelog - Version 1.17.0 - 4th May 2021
Changelog - Version 1.16.0 - February 3, 2021
Changelog - Version 1.15.0 - September 13, 2020
Changelog - Version 1.14.0 - November 1
Changelog - Version 1.13.0 - May 30th 2019
Changelog - Version 1.12.0
Changelog - Version 1.11.0
Changelog - Version 1.10.0
Changelog - Version 1.9.0
Version 1.8.0
Version 1.7.0

GitHubDiscordRSSSelect themeDarkLightAuto

MQTT Client Component

The MQTT Client Component sets up the MQTT connection to your broker.
If you are connecting to Home Assistant, you may prefer to use the native API,
in which case this is not needed.

MQTT is supported on the ESP32, ESP8266, BK72xx, LN882H and RTL87xx platforms.

WARNING

If you enable MQTT and you do not use the Api, you must
remove the api: configuration or set reboot_timeout: 0s, otherwise the ESP will
reboot every 15 minutes because no client connected to the native API.

# Example configuration entry

mqtt:

broker: 10.0.0.2

username: livingroom

password: !secret mqtt_password

NOTE

Support for esp-idf is still experimental. Please report issues you have with MQTT using the ESP-IDF framework.

Configuration variablesSection titled “Configuration variables”

broker (Required, string): The host of your MQTT broker.

enable_on_boot (Optional, boolean): If enabled, MQTT will be enabled on boot. Defaults to true.

port (Optional, int): The port to connect to. Defaults to 1883.

username (Optional, string): The username to use for
authentication. Empty (the default) means no authentication.

password (Optional, string): The password to use for
authentication. Empty (the default) means no authentication.

clean_session (Optional, boolean): Whether the broker will clean
the MQTT session after disconnect. Defaults to false.

client_id (Optional, string): The client id to use for opening
connections. See Defaults for more information.

discover_ip (Optional, boolean): If Home Assistant automatic device
discovery should be enabled. Defaults to true.

discovery (Optional, boolean): If Home Assistant automatic entity
discovery should be enabled. Defaults to true.

discovery_retain (Optional, boolean): Whether to retain MQTT
discovery messages so that entities are added automatically on Home
Assistant restart. Defaults to true.

discovery_prefix (Optional, string): The prefix to use for Home
Assistant’s MQTT discovery. Should not contain trailing slash.
Defaults to homeassistant.

discovery_unique_id_generator (Optional, string): The unique_id generator
to use. Can be one of legacy or mac. Defaults to legacy, which
generates unique_id in format ESP<component_type><default_object_id>.
mac generator uses format <mac_address>-<component_type>-<fnv1_hash(friendly_name)>.

discovery_object_id_generator (Optional, string): The object_id generator
to use. Can be one of none or device_name. Defaults to none which
does not generate object_id. device_name generator uses format <device_name>_<friendly_name>.

use_abbreviations (Optional, boolean): Whether to use
Abbreviations
in discovery messages. Defaults to true.

topic_prefix (Optional, string): The prefix used for all MQTT
messages. Should not contain trailing slash. Defaults to <APP_NAME>.
Use null to disable publishing or subscribing of any MQTT topic unless
it is explicitly configured.

log_topic (Optional, MQTTMessage): The topic to send MQTT log
messages to. Use null if you want to disable sending logs to MQTT.

The log_topic has an additional configuration option:

level (Optional, string): The log level to use for MQTT logs. See
Log Levels for options.

birth_message (Optional, MQTTMessage): The message to send when
a connection to the broker is established. See Last Will And Birth Messages for more information.

will_message (Optional, MQTTMessage): The message to send when
the MQTT connection is dropped. See Last Will And Birth Messages for more information.

shutdown_message (Optional, MQTTMessage): The message to send when
the node shuts down and the connection is closed cleanly. See Last Will And Birth Messages for more information.

certificate_authority (Optional, string): Only on ESP32. CA certificate in PEM format. See
TLS (ESP32) for more information.

TIP

For MQTT security recommendations including TLS configuration, see the Security Best Practices guide.

client_certificate (Optional, string): Only on esp32. Client certificate in PEM format.

client_certificate_key (Optional, string): Only on esp32. Client private key in PEM format.

skip_cert_cn_check (Optional, bool): Only on ESP32. Don’t verify if the common name in the server
certificate matches the value of broker.

idf_send_async (Optional, bool): Only on ESP32. If true publishing the message happens from a separate mqtt task.
The client only enqueues the message. Defaults to false.
The advantage of asynchronous publishing is that it doesn’t block the esphome main thread for potentially tens of seconds.
The disadvantage is additional memory usage for the thread.
Set this to true if you need to ensure that mqtt does not block the main thread, especially if you have poor network conditions.

reboot_timeout (Optional, Time): The amount of time to wait before rebooting when no
MQTT connection exists. Can be disabled by setting this to 0s. Defaults to 15min.

keepalive (Optional, Time): The time
to keep the MQTT socket alive, decreasing this can help with overall stability due to more
WiFi traffic with more pings. Defaults to 15 seconds.

on_connect (Optional, Automation): An action to be performed when a connection
to the broker is established.

on_disconnect (Optional, Automation): An action to be performed when the connection
to the broker is dropped.

on_message (Optional, Automation): An action to be
performed when a message on a specific MQTT topic is received. See on_message Trigger.

on_json_message (Optional, Automation): An action to be
performed when a JSON message on a specific MQTT topic is received. See on_json_message Trigger.

id (Optional, ID): Manually specify the ID used for code generation.

publish_nan_as_none (Optional, bool): Publish None instead of NaN to handle Unknown/Unavailable sensor
states in Home Assistant. Defaults to false.

wait_for_connection (Optional, bool): Blocks other components from starting until the MQTT connection is
established. Defaults to false.

MQTTMessageSection titled “MQTTMessage”

With the MQTT Message schema you can tell ESPHome how a specific MQTT message should be sent.
It is used in several places like last will and birth messages or MQTT log options.

# Simple:

some_option: topic/to/send/to

# Disable:

some_option:

# Advanced:

some_option:

topic: topic/to/send/to

payload: online

qos: 0

retain: true

Configuration options:

topic (Required, string): The MQTT topic to publish the message.

payload (Required, string): The message content. Will be filled by the actual payload with some
options, like log_topic.

qos (Optional, int): The Quality of Service
level of the topic. Defaults to 0.

retain (Optional, boolean): If the published message should
have a retain flag on or not. Defaults to true.

MQTT device discoverySection titled “MQTT device discovery”

The ESPHome device will respond to the following MQTT topics if mqtt.discover_ip is enabled.

esphome/discover (All ESPHome device will answer)

esphome/ping/<APP_NAME>

The response will be sent to esphome/discover/<APP_NAME> and is a JSON encoded message.

The MQTT device discovery is currently used for:

ESPHome dashboard (online / offline status)

ESPHome CLI (IP discovery; used to view logs and perform OTA uploads)

Home Assistant device discovery

Example Payload:

{

"ip": "192.168.0.122",

"name": "esp32-test",

"friendly_name": "Test Device",

"port": 6053,

"version": "2024.4.1",

"mac": "84fce6123456",

"platform": "ESP32",

"board": "esp32-c3-devkitm-1",

"network": "wifi",

"api_encryption": "Noise_NNpsk0_25519_ChaChaPoly_SHA256"

}

JSON keys:

ip (Required, ip): The IP address of the ESPHome device.

name (Required, string): Name of the device (esphome.name  ).

mac (Required, string): MAC address of the device.

board (Required, string): Board used for the device.

version (Required, string): ESPHome version.

port (Optional, port): Port of the ESPHome API (if enabled).

ipX (Optional, ip): Additional IP addresses (X is a number starting at 1).

friendly_name (Optional, string): Friendly name of the device (esphome.friendly_name  ).

platform (Optional, string): Platform of the device (e.g. ESP32 or ESP8266)

network (Optional, string): Network type.

project_name (Optional, string): esphome.project.name.

project_version (Optional, string): esphome.project.version.

project_version (Optional, string): dashboard_import.package_import_url.

api_encryption (Optional, string): API encryption type.

Using device discovery with Home AssistantSection titled “Using device discovery with Home Assistant”

MQTT can be used to automatically discover the ESPHome devices in Home Assistant.
This allows Home Assistant to find the ESPHome device and connect
to it via the ESPHome API which allows the usage
of more features then MQTT entity discovery alone (e.g. Bluetooth Proxy, Voice Assistant).

This can be achieved by enabling api and mqtt with mqtt.discover_ip enabled.
It may makes sense to disable mqtt.discovery since there will be no need to use the
MQTT entity discovery if Home Assistant will connect to the ESPHome API.

Example configuration:

api:

encryption:

key: "<secret>"

mqtt:

broker: 10.0.0.2

username: livingroom

password: !secret mqtt_password

discovery: False # disable entity discovery

discover_ip: True # enable device discovery

Using with Home Assistant MQTT entitiesSection titled “Using with Home Assistant MQTT entities”

Using ESPHome with Home Assistant is easy, simply setup an MQTT
broker (like mosquitto) and point both your
Home Assistant installation and ESPHome to that broker. Next, enable
discovery in your Home Assistant configuration with the following:

# Example Home Assistant configuration.yaml entry

mqtt:

broker: ...

And that should already be it 🎉 All devices defined through ESPHome should show up automatically
in the entities section of Home Assistant.

When adding new entities, you might run into trouble with old entities
still appearing in Home Assistant’s front-end. This is because in order
to have Home Assistant “discover” your devices on restart, all discovery
MQTT messages need to be retained. Therefore the old entities will also
re-appear on every Home Assistant restart even though they’re in
ESPHome anymore.

To fix this, ESPHome has a simple helper script that purges stale
retained messages for you:

Terminal window

esphome clean-mqtt configuration.yaml

With Docker:

Terminal window

docker run --rm -v "${PWD}":/config -it ghcr.io/esphome/esphome clean-mqtt configuration.yaml

This will remove all retained messages with the topic
<DISCOVERY_PREFIX>/+/NODE_NAME/#. If you want to purge on another
topic, simply add --topic <your_topic> to the command.

Home Assistant generates entity names for all discovered devices based on entity type and
entity name (e.g. sensor.uptime  ). Numeric suffixes are appended to entity names when
multiple devices use the same name for a sensor, making it harder to distinguish between
similar sensors on different devices. Home Assistant 2021.12 allows MQTT devices to change
this behaviour by specifying the object_id discovery attribute which replaces the sensor
name part of the generated entity name. Setting discovery_object_id_generator: device_name
in the ESPHome MQTT component configuration will cause Home Assistant to include device name
in the generated entity names (e.g. sensor.uptime becomes sensor.<device name>_uptime  ),
making it easier to distinguish the entities in various entity lists.

DefaultsSection titled “Defaults”

By default, ESPHome will prefix all messages with your node name or
topic_prefix if you have specified it manually. The client id will
automatically be generated by using your node name and adding the MAC
address of your device to it. Next, discovery is enabled by default with
Home Assistant’s default prefix homeassistant.

If you want to prefix all MQTT messages with a different prefix, like
home/living_room, you can specify a custom topic_prefix in the
configuration. That way, you can use your existing wildcards like
home/+/# together with ESPHome. All other features of ESPHome
(like availability) should still work correctly.

Last Will And Birth MessagesSection titled “Last Will And Birth Messages”

ESPHome uses the last willtestament
and birth message feature of MQTT to achieve availability reporting for
Home Assistant. If the node is not connected to MQTT, Home Assistant
will show all its entities as unavailable (a feature 😉).

By default, ESPHome will send a retained MQTT message to
<TOPIC_PREFIX>/status with payload online, and will tell the
broker to send a message <TOPIC_PREFIX>/status with payload
offline if the connection drops.

You can change these messages by overriding the birth_message and
will_message with the following options.

mqtt:

# ...

birth_message:

topic: myavailability/topic

payload: online

will_message:

topic: myavailability/topic

payload: offline

birth_message (Optional, MQTTMessage)

will_message (Optional, MQTTMessage)

If the birth message and last will message have empty topics or topics
that are different from each other, availability reporting will be
disabled.

TLS (ESP32)Section titled “TLS (ESP32)”

On ESP32, a TLS connection to an MQTT broker can be established.
The server’s CA certificate is required to validate the connection.

You have to download the server CA certificate in PEM format and add it to certificate_authority.
Usually these are .crt files and you can open them with any text editor.
Also make sure to change the port of the MQTT broker. Most brokers use port 8883 for TLS connections.

WARNING

MbedTLS, the library that handles TLS for the esp-idf, doesn’t validate wildcard certificates.

The Common Name check only works if the CN is explicitly reported in the certificate.

*.example.com -> Fail

mqtt.example.com -> Success

If a secure connection is necessary for your device, you really want to set:

skip_cert_cn_check: false

mqtt:

broker: test.mymqtt.local

port: 8883

discovery_prefix: ${mqtt_prefix}/homeassistant

log_topic: ${mqtt_prefix}/logs

# Evaluate carefully skip_cert_cn_check

skip_cert_cn_check: true

idf_send_async: false

certificate_authority: |

-----BEGIN CERTIFICATE-----

MIIEAzCCAuugAwIBAgIUBY1hlCGvdj4NhBXkZ/uLUZNILAwwDQYJKoZIhvcNAQEL

BQAwgZAxCzAJBgNVBAYTAkdCMRcwFQYDVQQIDA5Vbml0ZWQgS2luZ2RvbTEOMAwG

A1UEBwwFRGVyYnkxEjAQBgNVBAoMCU1vc3F1aXR0bzELMAkGA1UECwwCQ0ExFjAU

BgNVBAMMDW1vc3F1aXR0by5vcmcxHzAdBgkqhkiG9w0BCQEWEHJvZ2VyQGF0Y2hv

by5vcmcwHhcNMjAwNjA5MTEwNjM5WhcNMzAwNjA3MTEwNjM5WjCBkDELMAkGA1UE

BhMCR0IxFzAVBgNVBAgMDlVuaXRlZCBLaW5nZG9tMQ4wDAYDVQQHDAVEZXJieTES

MBAGA1UECgwJTW9zcXVpdHRvMQswCQYDVQQLDAJDQTEWMBQGA1UEAwwNbW9zcXVp

dHRvLm9yZzEfMB0GCSqGSIb3DQEJARYQcm9nZXJAYXRjaG9vLm9yZzCCASIwDQYJ

KoZIhvcNAQEBBQADggEPADCCAQoCggEBAME0HKmIzfTOwkKLT3THHe+ObdizamPg

UZmD64Tf3zJdNeYGYn4CEXbyP6fy3tWc8S2boW6dzrH8SdFf9uo320GJA9B7U1FW

Te3xda/Lm3JFfaHjkWw7jBwcauQZjpGINHapHRlpiCZsquAthOgxW9SgDgYlGzEA

s06pkEFiMw+qDfLo/sxFKB6vQlFekMeCymjLCbNwPJyqyhFmPWwio/PDMruBTzPH

3cioBnrJWKXc3OjXdLGFJOfj7pP0j/dr2LH72eSvv3PQQFl90CZPFhrCUcRHSSxo

E6yjGOdnz7f6PveLIB574kQORwt8ePn0yidrTC1ictikED3nHYhMUOUCAwEAAaNT

MFEwHQYDVR0OBBYEFPVV6xBUFPiGKDyo5V3+Hbh4N9YSMB8GA1UdIwQYMBaAFPVV

6xBUFPiGKDyo5V3+Hbh4N9YSMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQEL

BQADggEBAGa9kS21N70ThM6/Hj9D7mbVxKLBjVWe2TPsGfbl3rEDfZ+OKRZ2j6AC

6r7jb4TZO3dzF2p6dgbrlU71Y/4K0TdzIjRj3cQ3KSm41JvUQ0hZ/c04iGDg/xWf

+pp58nfPAYwuerruPNWmlStWAXf0UTqRtg4hQDWBuUFDJTuWuuBvEXudz74eh/wK

sMwfu1HFvjy5Z0iMDU8PUDepjVolOCue9ashlS4EB5IECdSR2TItnAIiIwimx839

LdUdRudafMu5T5Xma182OC0/u/xRlEm+tvKGGmfFcN0piqVl8OrSPBgIlb+1IKJE

m/XriWr/Cq4h/JfB7NTsezVslgkBaoU=

-----END CERTIFICATE-----

MQTT Component Base ConfigurationSection titled “MQTT Component Base Configuration”

All components in ESPHome that do some sort of communication through
MQTT can have some overrides for specific options.

name: "Component Name"

# Optional variables:

qos: 1

retain: true

availability:

topic: livingroom/status

payload_available: online

payload_not_available: offline

state_topic: livingroom/custom_state_topic

command_topic: livingroom/custom_command_topic

command_retain: false

Configuration variablesSection titled “Configuration variables”

name (Required, string): The name to use for the MQTT
Component.

qos (Optional, int): The Quality of Service
level for publishing. Defaults to 0.

retain (Optional, boolean): If all MQTT state messages should
be retained. Defaults to true.

discovery (Optional, boolean): Manually enable/disable
discovery for a component. Defaults to the global default.

subscribe_qos (Optional, int): The Quality of Service
level advertised in discovery for subscribing (only if discovery is enabled). Defaults to 0.

availability (Optional): Manually set what should be sent to
Home Assistant for showing entity availability. Default derived from
global birth/last will message.

state_topic (Optional, string, templatable): The topic to publish state
updates to. Defaults to
<TOPIC_PREFIX>/<COMPONENT_TYPE>/<COMPONENT_NAME>/state.

ESPHome will always publish a manually configured state topic, even if
the component is internal. Use null (or return "" in the lambda) to disable publishing the
component’s state.

command_topic (Optional, string, templatable): The topic to subscribe to for
commands from the remote. Defaults to
<TOPIC_PREFIX>/<COMPONENT_TYPE>/<COMPONENT_NAME>/command.

ESPHome will always subscribe to a manually configured command topic,
even if the component is internal. Use null (or return "" in the lambda) to disable subscribing
to the component’s command topic.

command_retain (Optional, boolean): Whether MQTT command messages
sent to the device should be retained or not. Default to false.

WARNING

When changing these options and you’re using MQTT discovery, you will need to restart Home Assistant.
This is because Home Assistant only discovers a device once in every Home Assistant start.

TriggersSection titled “Triggers”

on_connect TriggerSection titled “on_connect Trigger”

This trigger is activated when a connection to the MQTT broker is established. To retrieve if the session is present,
use a lambda template, it is available under the name session_present inside that lambda.
session_present indicates whether the broker has a persistent session for this client from a previous connection. When true,
the broker retained subscriptions and queued messages. When false, the session is new.

mqtt:

# ...

on_connect:

- switch.turn_on: switch1

- lambda: |-

ESP_LOGI("mqtt", "Session present: %s", session_present ? "true" : "false");

if (!session_present) {

// Do something if session is not present

}

on_disconnect TriggerSection titled “on_disconnect Trigger”

This trigger is activated when a connection to the MQTT broker is dropped. To retrieve the disconnect reason,
use a lambda template, the reason is available under the name reason inside that lambda.

mqtt:

# ...

on_disconnect:

- switch.turn_off: switch1

- lambda: |-

// reason is of type MQTTClientDisconnectReason

// Possible values:

//   TCP_DISCONNECTED (0)

//   MQTT_UNACCEPTABLE_PROTOCOL_VERSION (1)

//   MQTT_IDENTIFIER_REJECTED (2)

//   MQTT_SERVER_UNAVAILABLE (3)

//   MQTT_MALFORMED_CREDENTIALS (4)

//   MQTT_NOT_AUTHORIZED (5)

//   ESP8266_NOT_ENOUGH_SPACE (6)

//   TLS_BAD_FINGERPRINT (7)

//   DNS_RESOLVE_ERROR (8)

if (reason == mqtt::MQTTClientDisconnectReason::MQTT_NOT_AUTHORIZED) {

ESP_LOGE("mqtt", "Not authorized!");

}

on_message TriggerSection titled “on_message Trigger”

With this configuration option you can write complex automations whenever an MQTT
message on a specific topic is received. To use the message content, use a lambda
template, the message payload is available under the name x inside that lambda.

mqtt:

# ...

on_message:

topic: my/custom/topic

qos: 0

then:

- switch.turn_on: some_switch

Configuration variablesSection titled “Configuration variables”

topic (Required, string): The MQTT topic to subscribe to and listen for MQTT
messages on. Every time a message with this exact topic is received, the automation will trigger.

qos (Optional, int): The MQTT Quality of Service to subscribe to the topic with. Defaults
to 0.

payload (Optional, string): Optionally set a payload to match. Only if exactly the payload
you specify with this option is received, the automation will be executed.

NOTE

You can even specify multiple on_message triggers by using a YAML list:

mqtt:

on_message:

- topic: some/topic

then:

- # ...

- topic: some/other/topic

then:

- # ...

NOTE

This action can also be used in lambdas:

mqtt:

# Give the MQTT component an ID

id: mqtt_client

id(mqtt_client).subscribe("the/topic", [=](const std::string &topic, const std::string &payload) {

// do something with payload

});

on_json_message TriggerSection titled “on_json_message Trigger”

With this configuration option you can write complex automations whenever a JSON-encoded MQTT
message is received. To use the message content, use a lambda
template, the decoded message payload is available under the name x inside that lambda.

The x object is of type JsonObject by the ArduinoJson
library, and you can use all of the methods of that library to access data.

Basically, you can access elements by typing x["THE_KEY"] and save them into local variables.
Please note that it’s a good idea to check if the key exists in the Json Object by calling
containsKey first as the ESP will crash if an element that does not exist is accessed.

mqtt:

# ...

on_json_message:

topic: the/topic

then:

- light.turn_on:

id: living_room_lights

transition_length: !lambda |-

int length = 1000;

if (x.containsKey("length"))

length = x["length"];

return length;

brightness: !lambda "return x["bright"];"

effect: !lambda |-

const char *effect = "None";

if (x.containsKey("effect"))

effect = x["effect"];

return effect;

Configuration variablesSection titled “Configuration variables”

topic (Required, string): The MQTT topic to subscribe to and listen for MQTT
messages on. Every time a message with this exact topic is received, the automation will trigger.

qos (Optional, int): The MQTT Quality of Service to subscribe to the topic with. Defaults
to 0.

NOTE

Due to the way this trigger works internally it is incompatible with certain actions and will
trigger a compile failure. For example with the delay action.

NOTE

This action can also be used in lambdas:

mqtt:

# Give the MQTT component an ID

id: mqtt_client

id(mqtt_client).subscribe_json("the/topic", [=](const std::string &topic, JsonObject root) {

// do something with JSON-decoded value root

});

ActionsSection titled “Actions”

mqtt.publish ActionSection titled “mqtt.publish Action”

Publish an MQTT message on a topic using this action in automations.

on_...:

then:

- mqtt.publish:

topic: some/topic

payload: "Something happened!"

# Templated:

- mqtt.publish:

topic: !lambda |-

if (id(reed_switch).state) return "topic1";

else return "topic2";

payload: !lambda |-

return id(reed_switch).state ? "YES" : "NO";

Configuration variablesSection titled “Configuration variables”

topic (Required, string, templatable):
The MQTT topic to publish the message.

payload (Required, string, templatable): The message content.

qos (Optional, int, templatable): The Quality of
Service
level of the topic. Defaults to 0.

retain (Optional, boolean, templatable): If the published message should
have a retain flag on or not. Defaults to false.

NOTE

This action can also be written in lambdas:

mqtt:

# Give the MQTT component an ID

id: mqtt_client

id(mqtt_client).publish("the/topic", "The Payload");

mqtt.publish_json ActionSection titled “mqtt.publish_json Action”

Publish a JSON-formatted MQTT message on a topic using this action in automations.

The JSON message will be constructed using the ArduinoJson library.
In the payload option you have access to a root object which will represents the base object
of the JSON message. You can assign values to keys by using the root["KEY_NAME"] = VALUE; syntax
as seen below.

on_...:

then:

- mqtt.publish_json:

topic: the/topic

payload: |-

root["key"] = id(my_sensor).state;

root["greeting"] = "Hello World";

# Will produce:

# {"key": 42.0, "greeting": "Hello World"}

Configuration variablesSection titled “Configuration variables”

topic (Required, string, templatable):
The MQTT topic to publish the message.

payload (Required, lambda): The message content.

qos (Optional, int, templatable): The Quality of Service
level of the topic. Defaults to 0.

retain (Optional, boolean, templatable): If the published message should
have a retain flag on or not. Defaults to false.

NOTE

This action can also be written in lambdas:

mqtt:

# Give the MQTT component an ID

id: mqtt_client

id(mqtt_client).publish_json("the/topic", [=](JsonObject root) {

root["something"] = id(my_sensor).state;

});

mqtt.disable ActionSection titled “mqtt.disable Action”

This action turns off the MQTT component on demand.

on_...:

then:

- mqtt.disable:

NOTE

The configuration option enable_on_boot can be set to false if you do not want MQTT to be enabled on boot.

mqtt.enable ActionSection titled “mqtt.enable Action”

This action turns on the MQTT component on demand.

on_...:

then:

- mqtt.enable:

NOTE

The configuration option enable_on_boot can be set to false if you do not want MQTT to be enabled on boot.
mqtt.enable can be useful for custom setups. For example, if the broker name is negotiated dynamically and
saved in a global variable.

mqtt:

id: mqtt_id

broker: ""

enable_on_boot: False

globals:

- id: broker_address

type: std::string

restore_value: yes

max_restore_data_length: 24

initial_value: '"192.168.1.2"'

on_...:

then:

- lambda: !lambda id(mqtt_id).set_broker_address(id(broker_address));

- mqtt.enable:

ConditionsSection titled “Conditions”

mqtt.connected ConditionSection titled “mqtt.connected Condition”

This Condition checks if the MQTT client is currently connected to
the MQTT broker.

on_...:

if:

condition:

mqtt.connected:

then:

- logger.log: MQTT is connected!

NOTE

This action can also be written in lambdas:

mqtt:

# Give the MQTT component an ID

id: mqtt_client

if (id(mqtt_client)->is_connected()) {

// do something if MQTT is connected

}

See AlsoSection titled “See Also”

API Reference: mqtt_client.h