//! I2C configuration

use hal::i2c;

//#[allow(dead_code)]
//#[allow(missing_docs)]
//pub struct I2C {
//  peripheral: I2CPeripheral, // TODO(farcaller): clean up the warning
//  reg: &'static reg::I2C,
//}

/// Available UART peripherals.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum I2CPeripheral {I2C0, I2C1}

impl I2CPeripheral {
  fn reg(self) -> &'static reg::I2C {
    match self {
      I2CPeripheral::I2C0 => &reg::I2C0,
      I2CPeripheral::I2C1 => &reg::I2C1,
    }
  }
}

/// Structure describing a I2C instance.
#[derive(Clone, Copy)]
pub struct I2C {
  reg: &'static reg::I2C,
}

impl I2C {
  /// Returns platform-specific UART object that implements CharIO trait.
  pub fn new(peripheral: I2CPeripheral) -> I2C {
    let i2c = I2C {
      reg: peripheral.reg()
    };

    // TODO Init peripheral.
    i2c
  }
}

impl i2c::I2CMaster for I2C {
  fn write(&self, address: u16, value: u8) {
  }
  fn read(&self) -> u8 {
    0
  }
  fn start(&self){

  }
  fn stop(&self){

  }
}

/// Register definitions
pub mod reg {
  use volatile_cell::VolatileCell;
//  use core::ops::Drop;
//  use core::intrinsics::abort;

  ioregs!(I2C={
    0x0   => reg8 a0 {
      1..7  => ad,   //= address value
    },

    0x1   => reg8 f {
      0..5  => icr,  //= clock rate
      6..7  => mul,  //= multiplier factor
    },

    0x2   => reg8 c1 {
      7     => iicen,  //= I2C Enable
      6     => iicie,  //= I2C Interrupt Enable
      5     => mst,  //= Master Mode Select
      4     => tx,   //= Transmit Mode Select
      3     => txak { //= Transmit Acknowledge Enable
        0x0  => enable,   // An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set).
        0x1  => disable,  // No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set).
      },
      2     => rsta, //= Repeat START
      1     => wuen, //= Wakeup Enable
      0     => dmaen,  //= DMA Enable
    },

    0x3   => reg8 s {
      7     => tcf, //= Transfer Complete Flag
      6     => iaas, //= Addressed As A Slave
      5     => busy, //= Bus Busy
      4     => arbl, //= Arbitration Lost
      3     => ram,  //= Range Address Match
      2     => srw,  //= Slave Read/Write
      1     => iicif,  //= Interrupt Flag
      0     => rxak, //= Receive Acknowledge
    },

    0x4   => reg8 d {
      0..7  => data,
    },

    0x5   => reg8 c2 {
      7     => gcaen,  //= General Call Address Enable
      6     => adext,  //= Address Extension
      5     => hdrs,   //= High Drive Select
      4     => sbrc,   //= Slave Baud Rate Control
//        0 The slave baud rate follows the master baud rate and clock stretching may occur
//        1 Slave baud rate is independent of the master baud rate
      3     => rmen,   //= Range Address Matching Enable
      0..2  => ad,     //= Slave Address
                      //=   upper three bits of the slave address in the 10-bit address scheme
    },

    0x6   => reg8 flt {
      0..4  => flt,    //= I2C Programmable Filter Factor
                          //=   01-1Fh Filter glitches up to width of n bus clock cycles
    },

    0x7   => reg8 ra {
      1..7  => rad,  //=Range Slave Address
    },

    0x8   => reg8 smb {
      7     => fack,   //= Fast NACK/ACK Enable
      6     => alerten,    //= SMBus Alert Response Address Enable
      5     => siicaen,    //= Second I2C Address Enable
      4     => tcksel,   //= Timeout Counter Clock Select
      3     => sltf,   //= SCL Low Timeout Flag
      2     => shtf1,  //= SCL High Timeout Flag 1
      1     => shtf2,  //= SCL High Timeout Flag 2
      0     => shtf2ie,  //= SHTF2 Interrupt Enable
    },

    0x9   => reg8 a2 {
      1..7  => sad,  //= SMBus Address
    },

    0xa   => reg8 slth {
      0..7  => sslt, //= Most significant byte of SCL low timeout value that determines the timeout period of SCL low.
    },

    0xb   => reg8 sltl {
      0..7  => sslt,   //= Least significant byte of SCL low timeout value that determines the timeout period of SCL low.
    }
  });

  extern {
    #[link_name="k20_iomem_I2C0"] pub static I2C0: I2C;
    #[link_name="k20_iomem_I2C1"] pub static I2C1: I2C;
  }
}