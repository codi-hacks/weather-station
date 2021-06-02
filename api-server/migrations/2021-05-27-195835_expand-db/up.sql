CREATE TABLE "sensor_types" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    label VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE "sensors" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    alias VARCHAR NOT NULL,
    label VARCHAR NOT NULL,
    type_id UUID NOT NULL,
    station_id UUID NOT NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    FOREIGN KEY (type_id) REFERENCES sensor_types(id),
    FOREIGN KEY (station_id) REFERENCES stations(id)
);

CREATE TABLE "measurements" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    value NUMERIC NOT NULL,
    sensor_id UUID NOT NULL,
    created_at VARCHAR NOT NULL,
    FOREIGN KEY (sensor_id) REFERENCES sensors(id)
);