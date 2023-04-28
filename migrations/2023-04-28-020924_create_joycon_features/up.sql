CREATE TABLE joycon_data (
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

    
