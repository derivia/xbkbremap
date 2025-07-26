use crate::config::XboxButton;
use anyhow::{Context, Result};
use evdev::UinputAbsSetup;
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, BusType, EventType, InputEvent, InputId, Key,
};

pub struct VirtualController {
    device: VirtualDevice,
}

impl VirtualController {
    pub fn new() -> Result<Self> {
        let mut keys = AttributeSet::<Key>::new();
        keys.insert(Key::BTN_SOUTH); // A
        keys.insert(Key::BTN_EAST); // B
        keys.insert(Key::BTN_WEST); // X
        keys.insert(Key::BTN_NORTH); // Y
        keys.insert(Key::BTN_TL); // LB
        keys.insert(Key::BTN_TR); // RB
        keys.insert(Key::BTN_THUMBL); // LT
        keys.insert(Key::BTN_THUMBR); // RT
        keys.insert(Key::BTN_START);
        keys.insert(Key::BTN_SELECT);

        let ls_x_setup = UinputAbsSetup::new(
            AbsoluteAxisType::ABS_X,
            AbsInfo::new(0, -32768, 32767, 16, 128, 0),
        );
        let ls_y_setup = UinputAbsSetup::new(
            AbsoluteAxisType::ABS_Y,
            AbsInfo::new(0, -32768, 32767, 16, 128, 0),
        );
        let rs_x_setup = UinputAbsSetup::new(
            AbsoluteAxisType::ABS_RX,
            AbsInfo::new(0, -32768, 32767, 16, 128, 0),
        );
        let rs_y_setup = UinputAbsSetup::new(
            AbsoluteAxisType::ABS_RY,
            AbsInfo::new(0, -32768, 32767, 16, 128, 0),
        );
        let lt_setup =
            UinputAbsSetup::new(AbsoluteAxisType::ABS_Z, AbsInfo::new(0, 0, 1023, 0, 0, 0));
        let rt_setup =
            UinputAbsSetup::new(AbsoluteAxisType::ABS_RZ, AbsInfo::new(0, 0, 1023, 0, 0, 0));
        // HACK: Dpads as axis to work with xbox game pass
        let dpad_x_setup =
            UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0X, AbsInfo::new(0, -1, 1, 0, 0, 0));
        let dpad_y_setup =
            UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0Y, AbsInfo::new(0, -1, 1, 0, 0, 0));

        let input_id = InputId::new(
            BusType::BUS_USB,
            0x045e, // Microsoft Corporation Vendor ID
            0x028e, // Xbox 360 controller Product ID
            0x0110, // Version number
        );

        let device = VirtualDeviceBuilder::new()?
            .name("Microsoft X-Box 360 pad")
            // HACK: Use a spoofed vendor and product ID to work with xbox game pass
            .input_id(input_id)
            .with_keys(&keys)?
            .with_absolute_axis(&ls_x_setup)?
            .with_absolute_axis(&ls_y_setup)?
            .with_absolute_axis(&rs_x_setup)?
            .with_absolute_axis(&rs_y_setup)?
            .with_absolute_axis(&lt_setup)?
            .with_absolute_axis(&rt_setup)?
            .with_absolute_axis(&dpad_x_setup)?
            .with_absolute_axis(&dpad_y_setup)?
            .build()
            .context(
                "Failed to create virtual uinput device. Run with sudo or configure udev rules.",
            )?;

        Ok(Self { device })
    }

    pub fn handle_button_action(&mut self, button: XboxButton, value: i32) -> Result<()> {
        let event = match button {
            XboxButton::A => InputEvent::new(EventType::KEY, Key::BTN_SOUTH.0, value),
            XboxButton::B => InputEvent::new(EventType::KEY, Key::BTN_EAST.0, value),
            XboxButton::X => InputEvent::new(EventType::KEY, Key::BTN_WEST.0, value),
            XboxButton::Y => InputEvent::new(EventType::KEY, Key::BTN_NORTH.0, value),
            XboxButton::LB => InputEvent::new(EventType::KEY, Key::BTN_TL.0, value),
            XboxButton::RB => InputEvent::new(EventType::KEY, Key::BTN_TR.0, value),
            XboxButton::LS => InputEvent::new(EventType::KEY, Key::BTN_THUMBL.0, value),
            XboxButton::RS => InputEvent::new(EventType::KEY, Key::BTN_THUMBR.0, value),
            XboxButton::Start => InputEvent::new(EventType::KEY, Key::BTN_START.0, value),
            XboxButton::Select => InputEvent::new(EventType::KEY, Key::BTN_SELECT.0, value),

            XboxButton::DPadUp => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_HAT0Y.0,
                if value == 1 { -1 } else { 0 },
            ),
            XboxButton::DPadDown => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_HAT0Y.0,
                if value == 1 { 1 } else { 0 },
            ),
            XboxButton::DPadLeft => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_HAT0X.0,
                if value == 1 { -1 } else { 0 },
            ),
            XboxButton::DPadRight => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_HAT0X.0,
                if value == 1 { 1 } else { 0 },
            ),
            XboxButton::LT => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_Z.0,
                if value == 1 { 1023 } else { 0 },
            ),
            XboxButton::RT => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_RZ.0,
                if value == 1 { 1023 } else { 0 },
            ),

            XboxButton::LSUp => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_Y.0,
                if value == 1 { -32768 } else { 0 },
            ),
            XboxButton::LSDown => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_Y.0,
                if value == 1 { 32767 } else { 0 },
            ),
            XboxButton::LSLeft => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_X.0,
                if value == 1 { -32768 } else { 0 },
            ),
            XboxButton::LSRight => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_X.0,
                if value == 1 { 32767 } else { 0 },
            ),
            XboxButton::RSUp => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_RY.0,
                if value == 1 { -32768 } else { 0 },
            ),
            XboxButton::RSDown => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_RY.0,
                if value == 1 { 32767 } else { 0 },
            ),
            XboxButton::RSLeft => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_RX.0,
                if value == 1 { -32768 } else { 0 },
            ),
            XboxButton::RSRight => InputEvent::new(
                EventType::ABSOLUTE,
                AbsoluteAxisType::ABS_RX.0,
                if value == 1 { 32767 } else { 0 },
            ),
        };

        self.device
            .emit(&[event, InputEvent::new(EventType::SYNCHRONIZATION, 0, 0)])?;

        Ok(())
    }
}
