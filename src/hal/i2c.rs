// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*!
I2C interface.

I2CConf is a MCU-specific struct.

As I2C performs read and write as one operation, special care should be taken if
`write()` and `read()` methods are used with several devices on one I2C
peripheral.
*/

/// I2C trait.
pub trait I2CMaster {
  /// Writes a byte over I2C.
  ///
  /// It's implementation defined what happens if I2C is not configured to 8
  /// bits.
  fn write(&self, address: u16, value: u8);

  /// Reads a byte from I2C slave.
  ///
  /// This function returns the last byte received (I2C sends and receives data
  /// at the same time).
  fn read(&self) -> u8;

  /// Send start condition to I2C.
  fn start(&self);

  /// Send stop condition to I2C.
  fn stop(&self);
}
