-- Your SQL goes here


INSERT INTO stations(id, label, key) VALUES
  ('eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'Crater on Mars', 'P6JtQ1cNQlDcyaHGWp8XiUSUYfrioA'),
  ('f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'Trevors backyard', 'u7a7C0KBjaLuuXeRt04Hmk04WeNX6A');

INSERT INTO sensors(id, type_id, station_id, alias, label) VALUES
  ('819445e4-767c-41b1-8699-1a4a98333213', '6a3afc2e-f38e-4044-972d-49388f51053a', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'air_temp', 'Air temperature'),
  ('c3f775dc-4eec-4704-8d9f-f7c14adb0294', 'cb5bfac7-e839-4cea-898c-5db3a95fa638', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'humidity', 'Humidity'),
  ('04b1ece6-888a-4d9c-9840-8f636cf15f54', '012e0074-7b09-4b65-a3e0-bb27b9e934dd', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'pressure', 'Pressure'),
  ('66752c3e-49e2-46df-8dee-828e157acdad', '46a4fb66-4671-4d06-b531-1cad0c8629c5', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'altitude', 'Altitude'),
  ('7c876abd-29b9-49ad-a93d-e427601af247', '5332e7ca-2bfc-423e-a333-711f993bc3ab', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'signal', 'Signal strength'),
  ('847cb588-206f-4bfb-90c7-c991a58d116b', 'aaed6b1d-75b9-4130-929d-df16421f01f0', 'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c', 'voltage', 'Voltage'),
  ('1b708f78-cca1-4195-b43b-fece92a42443', '6a3afc2e-f38e-4044-972d-49388f51053a', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'air_temp', 'Air temperature'),
  ('3eb854e2-bb1e-4fb5-87a0-60fa18b3f346', 'cb5bfac7-e839-4cea-898c-5db3a95fa638', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'humidity', 'Humidity'),
  ('c977209b-088c-4cec-a1e6-d953c72a1907', '012e0074-7b09-4b65-a3e0-bb27b9e934dd', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'pressure', 'Pressure'),
  ('fc997f47-43e3-41e3-b482-dc902354e1e1', '46a4fb66-4671-4d06-b531-1cad0c8629c5', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'altitude', 'Altitude'),
  ('46cafd93-a003-45f1-bbdc-619fa0453360', '5332e7ca-2bfc-423e-a333-711f993bc3ab', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'signal', 'Signal strength'),
  ('b1d60cbd-27ca-4c06-a601-69db670d6b30', 'aaed6b1d-75b9-4130-929d-df16421f01f0', 'f9f32d13-d08e-49a0-b07f-a80cd98d27fe', 'voltage', 'Voltage');
