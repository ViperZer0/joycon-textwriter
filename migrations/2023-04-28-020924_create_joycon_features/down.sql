CREATE TABLE joycon_data_copy(
    symbol TEXT,
    training_num INT,
    time REAL,
    gyro_x REAL,
    gyro_y REAL,
    gyro_z REAL,
    accel_x REAL,
    accel_y REAL,
    accel_z REAL,
    PRIMARY KEY (symbol, training_num)
) STRICT;

INSERT INTO joycon_data_copy 
SELECT * FROM joycon_data;

DROP TABLE joycon_data;

ALTER TABLE joycon_data_copy RENAME TO joycon_data;

