void sleep(unsigned long start_time) {
  // calculate required sleep time and go to sleep
  long sleep_time = measurement_interval - millis(); // in milliseconds
  if (sleep_time < 100) sleep_time = 100; // set minimum sleep of 0.1 second

  Serial.print ("Going to sleep now for ");
  Serial.print((float)sleep_time / 1000, 3);
  Serial.println (" seconds.");
  Serial.print ("Time going to sleep=");
  Serial.print ((float)millis() / 1000, 3);
  Serial.print (" Event elapsed time=");
  Serial.println((float)(millis()-start_time)/1000,3);
  Serial.println();

  ESP.deepSleep(sleep_time * 5000); // convert to microseconds
}
