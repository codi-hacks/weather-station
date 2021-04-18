//----------------------------------------------------------------------------------------------------
//  Main microcontroller (ESP8266) and BME280 both sleep between measurements
//  BME280 is used in single shot mode ("forced mode")
//  Measurement read command is delayed,
//  By repeatedly checking the "measuring" bit of status register (0xF3) until ready
//----------------------------------------------------------------------------------------------------

// Wemos Mini D1 Pro pinout. This should have been provided by selecting the correct board but this
// board wasn't available when I looked for it so whatever this requires less dependency on the
// Arduino IDE configuration anyway.
// https://arduino-projekte.info/wp-content/uploads/2017/03/wemos_d1_mini_pro_pinout.png
byte PIN_D0 = 16;
byte PIN_D1 = 5;
byte PIN_D2 = 4;
byte PIN_D3 = 0;
byte PIN_D4 = 2;
byte PIN_D5 = 14;
byte PIN_D6 = 12;
byte PIN_D7 = 13;
byte PIN_D8 = 15;

// configuration constants
const bool bme280Debug = 0; // controls serial printing
// set to 1 to enable printing of BME280 or BMP280 transactions

//#include "user_interface.h" // https://arduino.stackexchange.com/questions/39957/esp8266-udp-multicast-doesnt-receive-packets/58268#58268
#include "config.h"
#include <Wire.h>
// BMP280 sensor library. Download from https://github.com/farmerkeith/BMP280-library
#include "farmerkeith_BMP280.h"
// WiFi documentation: https://arduino-esp8266.readthedocs.io/en/latest/esp8266wifi/station-class.html
#include <ESP8266WiFi.h>
// WifiUdp documentation: https://arduino-esp8266.readthedocs.io/en/latest/esp8266wifi/udp-examples.html
#include <WiFiUdp.h>
#include <ArduinoOTA.h>
#include <OneWire.h>
#include <DallasTemperature.h>
// Measurement subroutine, measure()
#include "measure.h"
// Deep sleep mode function, sleep()
#include "sleep.h"

void setup() {
  // Check this variable later to see how long the entire event took
  unsigned long start_time = millis();

  //**** Wi-fi setup *************************************************
  if (debug_mode == 1) {
    Serial.begin(115200);
    // Use one of these if you get errors with the faster rate
    //Serial.begin(9600);
    //Serial.begin(74880);
  }

  Serial.print("Connecting to WiFi");
  WiFi.mode(WIFI_STA);
  WiFi.hostname(hostname);
  WiFi.begin(ssid, pass);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println(" WiFi connected.");

  Serial.print("SSID:\t");
  Serial.println(WiFi.SSID());

  Serial.print("IP address:\t");
  Serial.println(WiFi.localIP());

  Serial.print("Gateway:\t");
  Serial.println(WiFi.gatewayIP());

  long rssi = WiFi.RSSI();
  Serial.print("Signal strength (RSSI):");
  Serial.print(rssi);
  Serial.println(" dBm");


  //**** Measuring and sleeping **************************************************
  measure();
  sleep(start_time);
} // end of void setup()


void loop() {
} // end of void loop()
