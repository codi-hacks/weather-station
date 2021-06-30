// https://github.com/arduino-libraries/Arduino_CRC32
#include <Arduino_CRC32.h>
Arduino_CRC32 crc32;

// BME280 configuration settings

// Temperature oversampling = 2
// No. of samples = 2 ^ (osrs_t-1) or 0 for osrs_t==0
const byte osrs_t = 2;

// Pressure oversampling = 16
// No. of samples = 2 ^ (osrs_p-1) or 0 for osrs_p==0
const byte osrs_p = 5;

// Humidity oversampling = 16
// No. of samples = 2 ^ (osrs_h-1) or 0 for osrs_h==0
const byte osrs_h = 5;


// Initialize DS18B20 water sensor(s)
OneWire oneWire(water_temperature_pin);
DallasTemperature sensors(&oneWire);

// Initialize bme280 sensor object bme0 of type bme280, base address
bme280 bme0(0, debug_mode);
// Create udp class instance
WiFiUDP udp;

void measure() {

  // Wire.begin(); // initialise I2C protocol - not needed here since it is in bmp library?

  //****** DS18B20 - water temperature ********************************************
  // Initialize one-wire DS18B20 sensors
  float water_temp = 0;
  if (enable_water_temperature == 1) {
    sensors.begin();
    sensors.requestTemperatures();
    water_temp = (sensors.getTempCByIndex(0));
  }

  //****** BME280 - Pressure, Temperature, Humidity, and calculated Altitude ******

  // Parameters are (osrs_t, osrs_p, mode, t_sb, filter, spi3W_en, osrs_h)
  // See BME280 data sheet for more detailed definitions. This is single shot mode with no filtering.
  bme0.begin(osrs_t, osrs_p, 1, 0, 0, 0, osrs_h);

  while (bme0.readRegister(0xF3) >> 3); // loop until F3bit 3 ==0, measurement is ready
  double temperature, pressure;
  double humidity = bme0.readHumidity (temperature, pressure); // measure pressure, temperature and humidity
  float altitude = bme0.calcAltitude (pressure);
  bme0.updateF4Control16xSleep();


  byte temperatureSamples = pow(2, osrs_t - 1);
  byte pressureSamples = pow(2, osrs_p - 1);
  byte humiditySamples = pow(2, osrs_h - 1);

  Serial.print ("Temperature samples=");
  Serial.print (temperatureSamples);
  Serial.print (" Pressure samples=");
  Serial.print (pressureSamples);
  Serial.print (" Humidity samples=");
  Serial.println (humiditySamples);

  Serial.print("Atm press = ");
  Serial.print(pressure, 2); // print with 2 decimal places
  Serial.print(" hPa. Temperature = ");
  Serial.print(temperature, 2); // print with 2 decimal places
  Serial.print( " deg C. Humidity = ");
  Serial.print(humidity, 2); // print with 2 decimal places
  Serial.print( " %RH. Altitude = ");
  Serial.println(altitude, 2); // print with 2 decimal places

  //****** Battery Voltage Monitoring *********************************************

  // Voltage divider
  // R1 = 220k + 100k + 220k (540k)
  // R2 = 100k
  float calib_factor = 5.28; // change this value to calibrate the battery voltage
  unsigned long raw_power = analogRead(A0);

  //****** UDP Datagram ************************************************************

  String message = "";
  message += "pressure=";
  message += String(pressure);
  if (enable_water_temperature == 1) {
    message += ",water_temp=";
    message += String(water_temp);
  }
  message += ",air_temp=";
  message += String(temperature);
  message += ",humidity=";
  message += String(humidity);
  message += ",altitude=";
  message += String(altitude);
  message += ",voltage=";
  message += String(raw_power * calib_factor/1024);
  message += ",signal=";
  message += WiFi.RSSI();
  message += ",id=";
  message += String(station_id);
  message += "#";

  // Hash the message with a secret key so we can identify the authenticity
  // of data source as the weather station we say it is.
  String string_to_hash = message + api_key;
  uint32_t const crc32_res = crc32.calc((uint8_t const *)string_to_hash.c_str(), string_to_hash.length());
  message += crc32_res;
  char const *buffer = message.c_str();

  Serial.print("Sending UDP datagram to ");
  Serial.print(server);
  Serial.print(":");
  Serial.println(port);
  udp.begin(port);
  udp.beginPacket(server, port);
  udp.write(buffer, message.length());

  udp.endPacket();
  //memset(buffer, 0, message.length());
  delay(1000);
}
