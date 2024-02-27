/// Marker for input only pins
pub struct ForwardInputPin;

/// Marker for output only pins
pub struct ForwardOutputPin;

/// Marker for input-output pins
pub struct ForwardIoPin;

/// Marker for full-blown I2C implemenations implemenating all embedded-hal 0.2 traits
pub struct ForwardFullI2c;

/// Marker for I2C implementations implementing just `Write` and `Read`
pub struct ForwardWriteReadI2c;

/// Marker for I2C implementations implemening `Write`, `Read`, and `WriteRead`
pub struct ForwardWriteReadWritereadI2c;
