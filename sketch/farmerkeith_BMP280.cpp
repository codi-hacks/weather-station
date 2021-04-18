/*
  https://github.com/farmerkeith/BMP280-library
  Revision 4c0011d8bd6aa96b0871bc7ed61de73003922627 (2019-01-09)
*/

/*
  farmerkeith_BMP280.cpp - library for the Bosch Sensortec BME280 and BMP280 pressure and temperature sensor
  created by farmerkeith, 15 september 2017
  Released into the public domain for personal use and not-for-profit projects.
  Expanded 16 December 2018 to support BME280 with humidity as well as pressure and temperature
  31 December 2018 added functions for compatability with other BME/P280 libraries
*/
// include guard
#ifndef farmerkeith_BMP280_cpp
#define farmerkeith_BMP280_cpp

#include "Arduino.h"
#include "farmerkeith_BMP280.h"
#include "Wire.h" // for I2C protocol

// *****************************************************
// constructors
// the constructors set up the I2C address and debug printing control
bme280::bme280 (byte Addr, byte debug) {       // constructor for class bme280
  address = bmp280Addr + Addr;
  bmp280Debug = debug;
} // end of bme280 constructor

bme280::bme280 (byte Addr) {                    // bme280 constructor with 1 parameter
  address = bmp280Addr + Addr;
  bmp280Debug = 0;
}
bme280::bme280 () {                             // bme280 constructor with no parameters
  address = bmp280Addr;
  bmp280Debug = 0;
}

bmp280::bmp280 (byte Addr, byte debug) { // constructor for class bmp280
  address = bmp280Addr + Addr;
  bmp280Debug = debug;
} // end of bmp280 constructor

bmp280::bmp280 (byte Addr) {             // constructor with 1 parameter
  address = bmp280Addr + Addr;
  bmp280Debug = 0;
}
bmp280::bmp280 () {                      // constructor with no parameters
  address = bmp280Addr;
  bmp280Debug = 0;
}

BMP280::BMP280() {                       // compatability
  bmp280();
}

BME280::BME280() {                       // compatability
  bme280();
}

// *****************************************************
// begin and init functions
// begin and init function check the hardware device type, read the calibration parameters
// and set the initial operating mode
bool bmp280::begin() {
  return begin(7, 7, 3, 0, 0, 0); // defaults are 16x; Normal mode; 0.5ms, no filter, I2C
}

bool bmp280::init(){
  return begin();
}

bool bme280::begin() {
  return begin(7, 7, 3, 0, 0, 0, 7); // defaults are 16x; Normal mode; 0.5ms, no filter, I2C
}

bool bme280::init(){
  return begin();
}

bool bmp280::begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en) {
  // get individual calibration constants from bmp280
  // also set control and configuration registers
  // see update F4 and F5 for details of parameters
  // return is true for bme280 and bmp280 devices
  // return is false for other devices
  Wire.begin(); // start I2C interface
  if (bmp280Debug) {
    Serial.print("(debug) Initialising BMP280 object ");
    Serial.print (address - bmp280Addr);
    Serial.print (" at I2C address 0x");
    Serial.println(address, HEX);
  }
  byte ID = readRegister(0xD0); // chip-ID is in register 0xD0
  if (bmp280Debug) Serial.print ("Hardware device is ");
  if (ID == 0x58 || ID == 0x56 || ID == 0x57) {
    if (bmp280Debug) Serial.print ("BMP280");
  }
  else if (ID == 0x60) {
    if (bmp280Debug) Serial.print ("BME280, use BME280 library to get humidity");
  }
  else {
    if (bmp280Debug) {
      Serial.print ("unknown, ID=");
      Serial.print (ID);
    }
    return false;
  }
  getBmpCalibratonData(); // includes conversion to integers and debug printing
  updateF4Control(osrs_t, osrs_p, mode);  // oversampling and mode
  updateF5Config(t_sb, filter, spi3W_en); // standby time, IIR filter, I2C select
  return true;
} // end of void bmp280::begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en)

bool bme280::begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en, byte osrs_h) {
  // get individual calibration constants from bme280
  // also set control and configuration registers
  // see update F2, F4 and F5 for details of parameters
  // return is true for bme280 and bmp280 devices
  // return is false for other devices
  Wire.begin(); // start I2C interface
  if (bmp280Debug) {
    Serial.print("Initialising BME280 object ");
    Serial.print (address - bmp280Addr);
    Serial.print (" at I2C address 0x");
    Serial.println(address, HEX);
  }

  byte ID = readRegister(0xD0); // chip-ID is in register 0xD0
  if (bmp280Debug) Serial.print ("Hardware device is ");
  if (ID == 0x58 || ID == 0x56 || ID == 0x57) {
    if (bmp280Debug) Serial.print ("BMP280, humidity readings will be 0 (invalid)");
  }
  else if (ID == 0x60) {
    if (bmp280Debug) Serial.print ("BME280");
  }
  else {
    if (bmp280Debug) {
      Serial.print ("unknown, ID=");
      Serial.print (ID);
    }
    return false;
  }
  getBmpCalibratonData(); // includes conversion to integers and debug printing
  getBmeCalibratonData(); // includes conversion to integers and debug printing
  updateF2Control(osrs_h);
  updateF4Control(osrs_t, osrs_p, mode);  // oversampling and mode
  updateF5Config(t_sb, filter, spi3W_en); // standby time, IIR filter, I2C select
  return true;
} // end of void bme280::begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en, byte osrs_h)

// *****************************************************
// temperature functions
double bmp280::readTemperature () {
  // function to read the temperature in BMP280, calibrate it and
  // return the value in Celsius

  // get temperature and pressure data
  byte data[3]; // array of 3 bytes used to store temperature data
  byte errorCode = readByteArray(0xFA, 3, data); // get 3 bytes from 0xFA (temperature)
  // Convert temperature data bytes to 20-bits within 32 bit integer
  long adc_t = (((long)(data[0] & 0xFF) * 65536) + ((long)(data[1] & 0xFF) * 256) + (long)(data[2] & 0xF0)) / 16;

  if (bmp280Debug) {
    Serial.print ("BMP280-");
    Serial.print (address - bmp280Addr); // index
    Serial.print (" raw temperature=");
    Serial.println (adc_t);
  }
  // Temperature offset calculation
  return calcTemperature (adc_t);
} // end of double readTemperature ()

double bmp280::calcTemperature (long adc_t, double &t_fine) { // private
  // uses stored calibration factors to convert raw temperature into degrees C
  // Temperature offset calculations
  double var1 = (((double)adc_t) / 16384.0 - ((double)dig_T1) / 1024.0) * ((double)dig_T2);
  double var2 = ((((double)adc_t) / 131072.0 - ((double)dig_T1) / 8192.0) *
                 (((double)adc_t) / 131072.0 - ((double)dig_T1) / 8192.0)) * ((double)dig_T3);
  t_fine = (long)(var1 + var2);
  return (double)(var1 + var2) / 5120.0;
} // end double bmp280::calcTemperature (long adc_t, double &t_fine)

double bmp280::calcTemperature (long adc_t) { // public
  // uses stored calibration factors to convert raw temperature into degrees C
  // Temperature offset calculations
  double t_fine;
  return calcTemperature (adc_t, t_fine); // t_fine not visible
} // end double bmp280::calcTemperature (long adc_t)

float bmp280::getTemperature(){
  return readTemperature();
}

// *****************************************************
// pressure functions

double bmp280::readPressure (double &temperature) {
  // function to read the pressure in BMP280, calibrate it and
  // return the value in kPa
  // also returns temperature
  long adc_p, adc_t;
  double t_fine;
  adc_p = readRawPressure (adc_t);
  temperature = calcTemperature (adc_t, t_fine);
  return calcPressure (adc_p, t_fine);
} // end of double readPressure (double &temperature)

double bmp280::readPressure () { // without temperature return variable
  double temperature;
  return readPressure(temperature); // temperature is discarded
} // end of double bmp280::readPressure

long bmp280::readRawPressure (long &adc_t) {
  // function to read the pressure in BMP280 without calibrating
  // return the value as a 32 bit integer
  // also returns raw temperature as a 32 bit integer

  byte data[6]; // array of 6 bytes used to store pressure and temperature data
  byte errorCode = readByteArray(0xF7, 6, data); // get 6 bytes from 0xF7 (pressure and temperature)

  // Convert pressure and temperature data to 19-bits
  long adc_p = (((long)(data[0] & 0xFF) * 65536) + ((long)(data[1] & 0xFF) * 256) + (long)(data[2] & 0xF0)) / 16;
  adc_t = (((long)(data[3] & 0xFF) * 65536) + ((long)(data[4] & 0xFF) * 256) + (long)(data[5] & 0xF0)) / 16;

  if (bmp280Debug) {
    Serial.print ("BMP280-");
    Serial.print (address - bmp280Addr); // index
    Serial.print (" raw pressure=");
    Serial.println (adc_p);
  }
  return adc_p;
} // end of long readRawPressure (long &adc_t)

double bmp280::calcPressure (long adc_p, double t_fine) {
  // uses stored calibration factors to convert raw pressure into hPa

  // Pressure offset calculations
  // this follows the "most accurate" version of code in BME280_MOD-1022.cpp
  double var1 = ((double)t_fine / 2.0) - 64000.0;
  double var2 = var1 * var1 * ((double)dig_P6) / 32768.0;
  var2 = var2 + var1 * ((double)dig_P5) * 2.0;
  var2 = (var2 / 4.0) + (((double)dig_P4) * 65536.0);
  var1 = (((double) dig_P3) * var1 * var1 / 524288.0 + ((double) dig_P2) * var1) / 524288.0;
  var1 = (1.0 + var1 / 32768.0) * ((double)dig_P1);
  if (var1 == 0.0) return 0; // avoid exception caused by division by zero
  double p = 1048576.0 - (double)adc_p;
  p = (p - (var2 / 4096.0)) * 6250.0 / var1;
  var1 = ((double) dig_P9) * p * p / 2147483648.0;
  var2 = p * ((double) dig_P8) / 32768.0;
  return (p + (var1 + var2 + ((double)dig_P7)) / 16.0) / 100; // pressure in hPa
} // end of double bmp280::calcPressure (long rawPressure, double t_fine)

uint32_t bmp280::getPressure(){ // pressure in Pa (1Pa = 0.01 hPa)
  return uint32_t(readPressure()*100);
}


// ********************************************
// humidity functions
double bme280::readHumidity () {               // function
  // function for humidity alone
  double temperature, pressure;
  return readHumidity(temperature, pressure); // temperature and pressure are discarded
} // end of double bme280::readHumidity ()

double bme280::readHumidity (double &temperature, double &pressure) { // function
  // function to read the humidity in BME280, calibrate it and
  // return the value in %RH
  // also returns temperature (Celsius) and pressure (HPa)
  long adc_h, adc_p, adc_t;
  double t_fine;
  adc_h = readRawHumidity (adc_t, adc_p);
  temperature = calcTemperature (adc_t, t_fine);
  pressure = calcPressure (adc_p, t_fine);
  return calcHumidity (adc_h, t_fine);
} // end of double bme280::readHumidity (double &temperature, double &pressure)

long bme280::readRawHumidity (long &rawTemperature, long &rawPressure) { // reads raw humidity, temperature and pressure
  // function to read the humidity, temperature and pressure in BME280 without calibrating
  // return the values as 32 bit signed integers
  byte data[8]; // array of 8 bytes used to store pressure and temperature data
  byte errorCode = readByteArray(0xF7, 8, data); // get 8 bytes from 0xF7 (pressure, temperature, humidity)
  // Convert pressure and temperature data to 19-bits
  rawPressure = (((long)(data[0] & 0xFF) * 65536) + ((long)(data[1] & 0xFF) * 256) + (long)(data[2] & 0xF0)) / 16;
  rawTemperature = (((long)(data[3] & 0xFF) * 65536) + ((long)(data[4] & 0xFF) * 256) + (long)(data[5] & 0xF0)) / 16;
  long rawHumidity = (((long)(data[6] & 0xFF) * 256) + (long)(data[7] & 0xFF));
  if (bmp280Debug) {
    Serial.print ("BME280-");
    Serial.print (address - bmp280Addr); // index
    Serial.print (" raw humidity=");
    Serial.println (rawHumidity);
  }
  return rawHumidity;
} // end of bme280::readRawHumidity

double bme280::calcHumidity(long rawHumidity, double t_fine) {
  // uses stored calibration factors to convert raw pressure into %RH
  // Returns humidity in %rH as as double. Output value of 46.332 represents 46.332 %rH

  // Humidity offset calculations
  // this follows the "most accurate" version of code in BME280_MOD-1022.cpp
  double var_H;
  var_H = (((double)t_fine) - 76800.0);
  var_H = (rawHumidity - (((double)dig_H4) * 64.0 + ((double)dig_H5) / 16384.0 * var_H)) *
          (((double)dig_H2) / 65536.0 * (1.0 + ((double)dig_H6) / 67108864.0 * var_H *
                                         (1.0 + ((double)dig_H3) / 67108864.0 * var_H)));
  var_H = var_H * (1.0 - ((double)dig_H1) * var_H / 524288.0);
  if (var_H > 100.0) {
    var_H = 100.0;
  } else if (var_H < 0.0) {
    var_H = 0.0;
  }
  return var_H;
} // end of double bme280::calcHumidity (long rawPressure, double t_fine)

uint32_t bme280::getHumidity(){                // relative humidity in percent
  return readHumidity ();
}

// *****************************************************
// utility functions
float bmp280::calcAltitude (double pressure, float seaLevelhPa) {
  // returns altitude in metres based on pressure and seaLevelpressure, both in hPa
  // standard seaLevelhPa = 1013.25
  return (float)44330 * (1.0 - pow(pressure / seaLevelhPa, 0.19026));
} // end of float bmp280::calcAltitude (double pressure, float seaLevelhPa)

float bmp280::calcAltitude (double pressure) {
  // returns altitude in metres based on pressure and standard seaLevelpressure, both in hPa
  // standard seaLevelhPa = 1013.25
  return (float)44330 * (1.0 - pow(pressure / 1013.25, 0.19026));
} // end of float bmp280::calcAltitude (double pressure, float seaLevelhPa)

float bmp280::calcNormalisedPressure (double pressure, float altitude) {
  // returns normalised pressure at sea level based on pressure and current altitude in metres
  return pressure / pow((1 - altitude * 0.0000225577), 5.25588);
} // end of float bmp280::calcNormalisedPressure (double pressure, float altitude)

// ********************************************
// configuration controls
byte bmp280::readF4Sleep() {
  // returns 6-bit variable from bit[7:2] of register 0xF4
  // read this value before begin function
  return readRegister(0xF4) >> 2;
} // end of byte bmp280::readF4Sleep()

byte bmp280::readF5Sleep() {
  // returns 6-bit variable from bit[7:2] of register 0xF5
  // read this value before begin function
  return readRegister(0xF5) >> 2;
} // end of byte bmp280::readF5Sleep()

void bme280::updateF2Control(byte osrs_h) {
  // general function to update control register at 0xF2
  // details of register 0xF2
  // osrs_h[2:0] F2bits[2:0] 000=skipped, 001=1 sample, 010=2 samples, 011=4 samples,
  // 100=8 samples, 101,110,111=16 samples
  byte F2value = (osrs_h & 0x7);   // construct bit pattern
  updateRegister(0xF2, F2value);
}  // end of void updateF2Control(byte osrs_h)

byte bmp280::updateF4Control(byte osrs_t, byte osrs_p, byte mode) {
  // general function to update control register at 0xF4
  // details of register 0xF4
  // mode[1:0] F4bits[1:0] 00=sleep, 01,10=forced, 11=normal
  // osrs_p[2:0] F4bits[4:2] 000=skipped, 001=16bit, 010=17bit, 011=18bit, 100=19bit, 101,110,111=20 bit
  // osrs_t[2:0] F4bits[7:5] 000=skipped, 001=16bit, 010=17bit, 011=18bit, 100=19bit, 101,110,111=20 bit
  byte F4value = (osrs_t & 0x7) << 5 | (osrs_p & 0x7) << 2 | (mode & 0x3); // construct bit pattern
  byte error = updateRegister(0xF4, F4value);
  return error;
}  // end of void updateF4Control(byte osrs_t, byte osrs_p, byte mode)

byte bmp280::updateF4Control16xNormal() {   // function
  byte error = updateF4Control(7, 7, 3); // temperature 20 bit, pressure 20 bit, mode Normal (continuous)
  return error;
} // end of  void bmp280::updateF4Control16xNormal()

byte bmp280::updateF4Control16xSleep() {   // function
  byte error = updateF4Control(7, 7, 0); // temperature 20 bit, pressure 20 bit, mode Sleep
  return error;
} // end of  void bmp280::updateF4Control16xSleep()

byte bmp280::updateF4ControlSleep(byte value) {
  // sets register 0xF4 bits [1:0] to 00 (sleep) and bits [7:2] to supplied value
  // suggested use: write this value just before deepSleep begins
  byte error = updateF4Control(value >> 3 & 0x07, value & 0x07, 0); // Sleep mode and 6 bits of value
  return error;
} // end of void bmp280::updateF4ControlSleep(byte value)

byte bmp280::updateF5Config(byte t_sb, byte filter, byte spi3W_en) {   // function
  // this configures register F5 for standby time, IIR filter constant and SPI/I2C use
  // details of register 0xF5
  // spi3w_en F5 bit [0] 0 use I2C, 1 enable SPI
  // F5 bit [1] not used
  // filter[2:0] F5 bits[4:2] 000=1(OFF), 001=2, 010=4, 011=8, 1xx=16
  // t_sb[2:0] F5 bits [7:5]
  // with bmp280: 000=0.5ms, 001=62.5ms, 010=125ms, 011=250ms, 100=500ms, 101=1000ms, 110=2000ms, 111=4000ms
  // with bme280: 000=0.5ms, 001=62.5ms, 010=125ms, 011=250ms, 100=500ms, 101=1000ms, 110=10ms, 111=20ms
  byte F5value = (t_sb & 0x07) << 5 | (filter & 0x07) << 2 | (spi3W_en & 0x03); // construct bit pattern
  byte error = updateRegister(0xF5, F5value);
  return error;
} // end of void bmp280::updateF5Config(byte t_sb, byte filter, byte spi3W_en)

byte bmp280::updateF5Config1msIIR16I2C() {   // function
  byte error = updateF5Config(0, 0, 0); // standby time 0.5ms, filter Off, I2C
  return error;
} // end of void bmp280::updateF5Contfig1msIIR16I2C()

byte bmp280::updateF5ConfigSleep(byte value) {
  // sets register 0xF5 bits [1:0] to 00 (I2C) and bits [7:2] to supplied value
  // suitable for saving value while BMP280 is sleeping
  // write this value just before deepSleep begins
  byte error = updateF5Config(value >> 3 & 0x07, value & 0x07, 0); // I2C and 6 bits of value
  return error;
} // end of void bmp280::updateF5ConfigSleep(byte value)

// ********************************************
// calibration functions
void bmp280::getBmpCalibratonData() { // function
  // get calibration data
  byte b1[24]; // array of 24 bytes used to store calibration bytes.
  byte errorCode = readByteArray(0x88, 24, b1); // 0x88 Hex is start address of calibration data

  // Convert the data (actually calibration coefficients)
  // temperature coefficients
  dig_T1 = b1[0] + (b1[1] * 256); // unsigned
  dig_T2 = b1[2] + (b1[3] * 256);
  if (b1[3] >> 7) dig_T2 = dig_T2 + 0xFFFF0000;
  dig_T3 = b1[4] + (b1[5] * 256);
  if (b1[5] >> 7) dig_T3 = dig_T3 + 0xFFFF0000;
  // pressure coefficients
  dig_P1 = (b1[6] & 0xFF) + ((b1[7] & 0xFF) * 256); // bitwise AND
  dig_P2 = b1[8] + (b1[9] * 256);
  if (b1[9] >> 7) dig_P2 = dig_P2 + 0xFFFF0000;
  dig_P3 = b1[10] + (b1[11] * 256);
  if (b1[11] >> 7) dig_P3 = dig_P3 + 0xFFFF0000;
  dig_P4 = b1[12] + (b1[13] * 256);
  if (b1[13] >> 7) dig_P4 = dig_P4 + 0xFFFF0000;
  dig_P5 = b1[14] + (b1[15] * 256);
  if (b1[15] >> 7) dig_P5 = dig_P5 + 0xFFFF0000;
  dig_P6 = b1[16] + (b1[17] * 256);
  if (b1[17] >> 7) dig_P6 = dig_P6 + 0xFFFF0000;
  dig_P7 = b1[18] + (b1[19] * 256);
  if (b1[19] >> 7) dig_P7 = dig_P7 + 0xFFFF0000;
  dig_P8 = b1[20] + (b1[21] * 256);
  if (b1[21] >> 7) dig_P8 = dig_P8 + 0xFFFF0000;
  dig_P9 = b1[22] + (b1[23] * 256);
  if (b1[23] >> 7) dig_P9 = dig_P9 + 0xFFFF0000;

  if (bmp280Debug) {
    Serial.println ("(debug) Calibration parameters");
    Serial.print ("Temperature dig_T1="); Serial.print (dig_T1);
    Serial.print (" dig_T2="); Serial.print (dig_T2);
    Serial.print (" dig_T3="); Serial.println (dig_T3);
    Serial.print ("Pressure    dig_P1="); Serial.print (dig_P1);
    Serial.print (" dig_P2="); Serial.print (dig_P2);
    Serial.print (" dig_P3="); Serial.print (dig_P3);
    Serial.print (" dig_P4="); Serial.println (dig_P4);
    Serial.print (" dig_P5="); Serial.print (dig_P5);
    Serial.print (" dig_P6="); Serial.print (dig_P6);
    Serial.print (" dig_P7="); Serial.print (dig_P7);
    Serial.print (" dig_P8="); Serial.print (dig_P8);
    Serial.print (" dig_P9="); Serial.println (dig_P9);
    Serial.print ("End of calibration parameters for BMP280-");
    Serial.println (address - bmp280Addr);
  } // end of if (bmp280Debug)

} // end of void bmp280::getBmpCalibratonData()

void bme280::getBmeCalibratonData() { // function
  // get calibration data
  dig_H1 = readRegister(0xA1); // dig_H1 is a single byte separated from the others

  // next read the 7 bytes of contiguous humidity calibration data
  byte b1[7]; // array of 7 bytes used to store humidity calibration bytes.
  byte errorCode = readByteArray(0xE1, 7, b1);

  // Convert the data (actually calibration coefficients)
  // humidity coefficients
  dig_H2 = b1[0] + (b1[1] * 256); // dig_H2 is int16_t
  // coming from 0xE1 and 0xE2
  dig_H3 = b1[2];  // dig_H3 is uint8_t
  // coming from 0xE3
  dig_H4 = b1[3] * 16 + (b1[4] && 0x0F); // dig_H4 is uint16_t
  // bits [11:4] coming from 0xE4 and
  // bits [3:0] coming from bits [3:0] of 0xE5
  dig_H5 = b1[5] * 16 + (b1[4] && 0xF0) / 16; // dig_H5 is uint16_t
  // bits [11:4] coming from 0xE6 and
  // bits [3:0] coming from bits [7:4] of 0xE5
  dig_H6 = b1[6];   // dig_H5 is int8_t
  // coming from 0xE7
  if (bmp280Debug) {
    Serial.println ("Humidity Calibration parameters");
    Serial.print ("Pressure    dig_H1="); Serial.print (dig_H1);
    Serial.print (" dig_H2="); Serial.print (dig_H2);
    Serial.print (" dig_H3="); Serial.println (dig_H3);
    Serial.print ("            dig_H4="); Serial.print (dig_H4);
    Serial.print (" dig_H5="); Serial.print (dig_H5);
    Serial.print (" dig_H6="); Serial.print (dig_H6);
    Serial.print ("End of humidity calibration parameters for BME280-");
    Serial.println (address - bmp280Addr);
  } // end of if (bmp280Debug)
} // end of void bme280::getBmeCalibratonData()

// *****************************************************
// general tools
byte bmp280::readRegister(byte reg) {
  // general function to read control register
  noInterrupts(); // disable interrupts
  Wire.beginTransmission(address); // start I2C transmission
  Wire.write(reg); // address of register to control oversampling and power mode
  byte error = Wire.endTransmission();
  if (error && bmp280Debug) {
    Serial.print ("I2C error with address ");
    Serial.print (address, HEX);
    Serial.print (" error code= ");
    Serial.print (error);
    Serial.print (" millis= ");
    Serial.println (millis());
  } // end of if (error)
  // Request 1 byte of data
  Wire.requestFrom(address, 1);
  byte data = Wire.read();
  interrupts(); // enable interrupts
  /*  if (bmp280Debug) {
      Serial.print ("BMP280-");
      Serial.print (address - bmp280Addr);
      Serial.print (" register ");
      Serial.print (reg, HEX);
      Serial.print (" value ");
      Serial.println(data);
    } // end of if (bmp280Debug)
  */
  return (data);
}  // end of void readRegister(byte reg)

byte bmp280::updateRegister(byte reg, byte value) {   // function
  noInterrupts(); // disable interrupts
  Wire.beginTransmission(address);
  // Select register
  Wire.write(reg); // address of register
  Wire.write(value); //
  // Stop I2C Transmission
  byte error = Wire.endTransmission(); // end of write
  interrupts(); // enable interrupts
  if (error && bmp280Debug) {
    Serial.print ("I2C error with address ");
    Serial.print (address, HEX);
    Serial.print (" error code= ");
    Serial.print (error);
    Serial.print (" millis= ");
    Serial.println (millis());
  } // end of if (error)
  if (bmp280Debug) {
    Serial.print ("BMP280-");
    Serial.print (address - bmp280Addr);
    Serial.print (" register ");
    Serial.print (reg, HEX);
    Serial.print (" updated to ");
    Serial.print(value);
    Serial.print (" = HEX ");
    Serial.println(value, BIN);
  } // end of if (bmp280Debug)
  return error;
} // end of void bmp280::updateRegister(byte reg, byte value)

byte bmp280::readByteArray(byte reg, byte length, byte data[]) { // function
  // read an array of length starting from register address reg
  // and put it into array data[]
  // returns an error code (0 for no error)
  // byte b1[24]; // array of 24 bytes used to store calibration bytes.
  // Start I2C Transmission
  noInterrupts(); // disable interrupts
  Wire.beginTransmission(address);
  byte error = Wire.endTransmission();
  if (error && bmp280Debug) {
    Serial.print ("I2C error with address ");
    Serial.print (address, HEX);
    Serial.print (" error code= ");
    Serial.print (error);
    Serial.print ("millis= ");
    Serial.println (millis());
    return error;
  } // end of if (error)

  Wire.beginTransmission(address);
  Wire.write(reg); // reg is start address of the data
  Wire.endTransmission();

  // Request "length" bytes of data
  Wire.requestFrom(address, length);
  byte byteCount = Wire.available();
  if (byteCount == length)   { // correct answer
    if (bmp280Debug) {
      Serial.print ("\n(debug) getting ");
      Serial.print (byteCount);
      Serial.println (" bytes");
    } // end of if (bmp280Debug)
  } else {                    // byteCount does not match length
    if (bmp280Debug) {
      Serial.print ("byteCount not correct, but is ");
      Serial.println (byteCount);
    } // end of if (bmp280Debug)
    return 100 + byteCount; // 100 offset to avoid confusion with error code
  } // end of if bytecount == length)

  for (int i = 0; i < length; i++)  { // reads "length" bytes
    data[i] = Wire.read();
  } // end of for (int i = 0; i < length; i++)
  interrupts(); // enable interrupts
  return error;
} // end of bmp280::readByteArray

#endif // for #ifndef farmerkeith_BMP280_cpp
// end of file

