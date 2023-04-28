// @generated automatically by Diesel CLI.

diesel::table! {
    joycon_data (symbol, training_num) {
        symbol -> Text,
        training_num -> Integer,
        time -> Nullable<Float>,
        gyro_X -> Nullable<Float>,
        gyro_Y -> Nullable<Float>,
        gyro_Z -> Nullable<Float>,
        accel_X -> Nullable<Float>,
        accel_Y -> Nullable<Float>,
        accel_Z -> Nullable<Float>,
    }
}
