CREATE TABLE stations (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  label VARCHAR NOT NULL,
  key VARCHAR NOT NULL
);

CREATE TABLE sensor_types (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  label VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE sensors (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  alias VARCHAR NOT NULL,
  label VARCHAR NOT NULL,
  type_id UUID NOT NULL,
  station_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY (type_id) REFERENCES sensor_types(id),
  FOREIGN KEY (station_id) REFERENCES stations(id)
);

CREATE TABLE measurements (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  value NUMERIC NOT NULL,
  sensor_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY (sensor_id) REFERENCES sensors(id)
);

INSERT INTO sensor_types(id, label, description) VALUES
  ('6a3afc2e-f38e-4044-972d-49388f51053a', 'temperature', 'celsius'),
  ('cb5bfac7-e839-4cea-898c-5db3a95fa638', 'humidity',    'relative humitidy'),
  ('012e0074-7b09-4b65-a3e0-bb27b9e934dd', 'pressure',    'hectopascal'),
  ('46a4fb66-4671-4d06-b531-1cad0c8629c5', 'elevation',   'meters above sea level'),
  ('5332e7ca-2bfc-423e-a333-711f993bc3ab', 'signal',      'dbm'),
  ('aaed6b1d-75b9-4130-929d-df16421f01f0', 'voltage',     '18650 voltage');
