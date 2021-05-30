CREATE TABLE "sensors" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    alias VARCHAR NOT NULL,
    label VARCHAR NOT NULL,
    type_id UUID NOT NULL DEFAULT uuid_generate_v4 (),
    station_id UUID NOT NULL DEFAULT uuid_generate_v4 (),
    /*created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()*/
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL
);

CREATE TABLE "measurements" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    value integer NOT NULL,
    sensor_id UUID NOT NULL DEFAULT uuid_generate_v4 (),
    created_at VARCHAR NOT NULL
);

CREATE TABLE "sensor_types" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    label VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);
