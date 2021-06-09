-- This file should undo anything in `up.sql`

DELETE FROM stations
WHERE id IN (
  'eb54ffd7-34fb-4599-a5df-efdd2b8bf46c',
  'f9f32d13-d08e-49a0-b07f-a80cd98d27fe')
RETURNING *;

DELETE FROM sensors
WHERE id IN (
  '819445e4-767c-41b1-8699-1a4a98333213',
  'eb7468b8-b43c-47f1-968e-aabccbff8da0',
  'c3f775dc-4eec-4704-8d9f-f7c14adb0294',
  '04b1ece6-888a-4d9c-9840-8f636cf15f54',
  '66752c3e-49e2-46df-8dee-828e157acdad',
  '7c876abd-29b9-49ad-a93d-e427601af247',
  '847cb588-206f-4bfb-90c7-c991a58d116b',
  '1b708f78-cca1-4195-b43b-fece92a42443',
  '3eb854e2-bb1e-4fb5-87a0-60fa18b3f346',
  'c977209b-088c-4cec-a1e6-d953c72a1907',
  'fc997f47-43e3-41e3-b482-dc902354e1e1',
  '46cafd93-a003-45f1-bbdc-619fa0453360',
  'b1d60cbd-27ca-4c06-a601-69db670d6b30')
RETURNING *;
