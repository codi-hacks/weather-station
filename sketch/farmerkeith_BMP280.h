/*
  https://github.com/farmerkeith/BMP280-library
  Revision 4c0011d8bd6aa96b0871bc7ed61de73003922627 (2019-01-09)
*/

/*
  farmerkeith_BMP280.h - library for the Bosch Sensortec BMP280 pressure and temperature sensor
  created by Keith Hungerford, 15 september 2017
  Updated for bme280 an reorganising code 18 Dec 2018
  Released into the public domain for personal use and not-for-profit projects.
*/
#ifndef farmerkeith_BMP280_h
#define farmerkeith_BMP280_h

#include "Arduino.h"

class bmp280 { // for use with BMP280 sensors
  public:
    // constructors
    bmp280 (byte Addr, byte debug);
    bmp280 (byte Addr);
    bmp280 ();

    // begin and init functions
    bool begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en);
    bool begin();                // get calibration factors, set configuration
    bool init();
    // temperature functions
    double readTemperature ();   // returns degrees Celsius
    float getTemperature();      // compatability function, returns degrees Celsius
    double calcTemperature (long rawTemperature); // returns degrees Celsius
    double calcTemperature (long rawTemperature, double &t_fine); // returns degrees Celsius
    // pressure functions
    double readPressure (); // pressure in hPa
    double readPressure (double &temperature); // returns pressure in hPa, temperature in degrees Celsius
    long readRawPressure (long &rawTemperature); // returns raw pressure uncalibrated
    double calcPressure (long rawPressure, double t_fine); // returns pressure in hPa
    uint32_t getPressure(); // compatability function, returns pressure in Pa (1Pa = 0.01 hPa)
    // utility functions
    float calcAltitude (double pressure, float seaLevelhPa); // hPa, returns meters
    float calcAltitude (double pressure); // hPa, standard seaLevelPressure assumed, returns meters
    float calcNormalisedPressure (double pressure, float altitude); // hPa, meters
    // configuration controls
    byte readF4Sleep();           // function
    byte readF5Sleep();           // function
    byte updateF4Control(byte osrs_t, byte osrs_p, byte mode); // function
    byte updateF4Control16xNormal();       // function
    byte updateF4Control16xSleep();        // function
    byte updateF4ControlSleep(byte value); // function
    byte updateF5Config(byte t_sb, byte filter, byte spi3W_en);// function
    byte updateF5Config1msIIR16I2C();      // function
    byte updateF5ConfigSleep(byte value);  // function
    // calibration functions
    void getBmpCalibratonData();            // function
    // general tools
    byte readRegister(byte reg);           // function
    byte updateRegister(byte reg, byte value); // function
    byte readByteArray(byte reg, byte length, byte data[]); // function
    // public data and variables
    byte bmp280Debug = 0;
    // BMP280 I2C address is 0x76(108) when pin SDO is connected to GND
    // or 0x77(109) when pin SDO is connected to Vddio (+3.3V)
    const byte bmp280Addr = 0x76 ; // base address
    byte address;                  // base address + device index
  private:
    // private variables
    uint16_t dig_T1, dig_P1;                // temperature and pressure calibration
    int16_t dig_T2, dig_T3;                 // temperature calibration
    int16_t dig_P2, dig_P3, dig_P4, dig_P5; // pressure calibration
    int16_t dig_P6, dig_P7, dig_P8, dig_P9; // pressure calibration

}; // end of class bmp280

class BMP280 : public bmp280 { // compatability version
  public:
    BMP280 ();
};

class bme280 : public bmp280 {             // for use with bme280 sensors
    // class bme280 inherits all the data and functions of bmp280
  public:
    // constructors
    bme280 (byte Addr, byte debug);
    bme280 (byte Addr);
    bme280 ();
    // begin and init functions
    bool begin();                          // get calibration factors, set configuration
    bool begin(byte osrs_t, byte osrs_p, byte mode, byte t_sb, byte filter, byte spi3W_en, byte osrs_h);
    bool init();
    // humidity functions
    double readHumidity ();                // relative humidity in percent
    double readHumidity (double &temperature, double &pressure);  // RH%, Celsius, hPa
    long readRawHumidity (long &rawTemperature, long &rawPressure); // reads raw humidity, temperature and pressure
    double calcHumidity(long rawHumidity, double t_fine); // convert raw humidity code into %RH
    uint32_t getHumidity();                // for compatability. Relative humidity in percent
    // configuration controls
    void updateF2Control(byte osrs_h);     // function

  private:
    // calibration functions
    void getBmeCalibratonData();           // function
    // variables
    uint8_t  dig_H1, dig_H3;           // humidity calibration
    int16_t  dig_H2, dig_H4, dig_H5;   // humidity calibration
    int8_t   dig_H6;                   // humidity calibration
}; // end of class bme280

class BME280 : public bme280 { // compatability version
  public:
    BME280 ();
};

#endif // for #ifndef farmerkeith_BMP280_h
// end of file

