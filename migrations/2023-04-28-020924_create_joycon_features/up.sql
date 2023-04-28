CREATE TABLE joycon_data (
    symbol TEXT,
    training_num INT,
    time REAL,
    gyro_X REAL,
    gyro_Y REAL,
    gyro_Z REAL,
    accel_X REAL,
    accel_Y REAL,
    accel_Z REAL,
    PRIMARY KEY (symbol, training_num)
) STRICT;

    
