CREATE TABLE "stations" (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4 (),
    label VARCHAR NOT NULL,
    key VARCHAR NOT NULL
);
