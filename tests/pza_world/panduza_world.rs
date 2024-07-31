use std::fmt::Debug;


// use super::api_dio::PicohaDioRequest;
// use super::api_dio::RequestType;

use cucumber::World;


use panduza::ReactorSettings;
use panduza::SyncReactor;


// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(World)]
pub struct PanduzaWorld {
    pub reactor: Option<SyncReactor>,
    // pub serial_settings: SerialSettings,
    // pub serial_stream: Option<SerialStream>,
    // // Accumulated incoming data buffer
    // pub in_buf: [u8; 512],
    // // Keep track of number of data in the buffer
    // pub in_buf_size: usize,

    // decode_buffer: serial_line_ip::DecoderBuffer<512>,

    // pub last_answer: Option<PicohaDioAnswer>,
}

impl Debug for PanduzaWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanduzaWorld")
            // .field("usb_settings", &self.usb_settings)
            // .field("serial_settings", &self.serial_settings)
            // .field("serial_stream", &self.serial_stream)
            // .field("in_buf", &self.in_buf)
            // .field("in_buf_size", &self.in_buf_size)
            // // .field("decode_buffer", &self.decode_buffer)
            // .field("last_answer", &self.last_answer)
            .finish()
    }
}

impl PanduzaWorld {


  


}

impl std::default::Default for PanduzaWorld {
    fn default() -> Self {
        // let usb_s = UsbSettings::new().set_vendor(0x16c0).set_model(0x05E1);
        // let serial_s = SerialSettings::new()
        //     .set_port_name_from_usb_settings(&usb_s)
        //     .unwrap()
        //     .set_baudrate(9600)
        //     .set_read_timeout(std::time::Duration::from_secs(5));

        PanduzaWorld {
            reactor: None,
            // usb_settings: usb_s,
            // serial_settings: serial_s,
            // serial_stream: None,
            // in_buf: [0u8; 512],
            // in_buf_size: 0,
            // last_answer: None,
            // decode_buffer: serial_line_ip::DecoderBuffer::new(),
        }
    }
}
