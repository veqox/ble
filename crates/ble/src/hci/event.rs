use core::fmt::Debug;

use macros::{FromU8, IntoU8};
use utils::reader::Reader;

use super::{
    HCIEventPacket,
    gap::{AdvertisingData, AdvertisingDataType},
};

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7 | page 2240
// Events
#[derive(Debug, IntoU8, FromU8)]
#[repr(u8)]
pub enum HCIEventCode {
    DisconnectionComplete = 0x05, // 7.7.5
    CommandComplete = 0x0E,       // 7.7.14
    LEMetaEvent = 0x3E,           // 7.7.65
}

#[derive(Debug, IntoU8, FromU8)]
#[repr(u8)]
pub enum SubeventCode {
    ConnectionComplete = 0x01,                        // 7.7.65.1
    AdvertisingReport = 0x02,                         // 7.7.65.2
    ConnectionUpdateComplete = 0x03,                  // 7.7.65.3
    ReadRemoteFeaturesPage0Complete = 0x04,           // 7.7.65.4
    LongTermKeyRequest = 0x05,                        // 7.7.65.5
    RemoteConnectionParameterRequest = 0x06,          // 7.7.65.6
    DataLengthChange = 0x07,                          // 7.7.65.7
    ReadLocalP256PublicKeyComplete = 0x08,            // 7.7.65.8
    GenerateDHKeyComplete = 0x09,                     // 7.7.65.9
    EnhancedConnectionCompleteV1 = 0x0A,              // 7.7.65.10
    EnhancedConnectionCompleteV2 = 0x29,              // 7.7.65.10
    DirectedAdvertisingReport = 0x0B,                 // 7.7.65.11
    PHYUpdateComplete = 0x0C,                         // 7.7.65.12
    ExtendedAdvertisingReport = 0x0D,                 // 7.7.65.13
    PeriodicAdvertisingSyncEstablished = 0x0E,        // 7.7.65.14
    PeriodicAdvertisingReport = 0x0F,                 // 7.7.65.15
    PeriodicAdvertisingSyncLost = 0x10,               // 7.7.65.16
    ScanTimeout = 0x11,                               // 7.7.65.17
    AdvertisingSetTerminated = 0x12,                  // 7.7.65.18
    ScanRequestReceived = 0x13,                       // 7.7.65.19
    ChannelSelectionAlgorithm = 0x14,                 // 7.7.65.20
    ConnectionlessIQReport = 0x15,                    // 7.7.65.21
    ConnectionIQReport = 0x16,                        // 7.7.65.22
    CTERequestFailed = 0x17,                          // 7.7.65.23
    PeriodicAdvertisingSyncTransferReceivedV1 = 0x18, // 7.7.65.24
    PeriodicAdvertisingSyncTransferReceivedV2 = 0x26, // 7.7.65.24
    CISEstablishedV1 = 0x19,                          // 7.7.65.25
    CISEstablishedV2 = 0x2A,                          // 7.7.65.25
    CISRequest = 0x1A,                                // 7.7.65.26
    CreateBIGComplete = 0x1B,                         // 7.7.65.27
    TerminateBIGComplete = 0x1C,                      // 7.7.65.28
    BIGSyncEstablished = 0x1D,                        // 7.7.65.29
    BIGSyncLost = 0x1E,                               // 7.7.65.30
    RequestPeerSCAComplete = 0x1F,                    // 7.7.65.31
    PathLossThreshold = 0x20,                         // 7.7.65.32
    TransmitPowerReporting = 0x21,                    // 7.7.65.33
    BIGInfoAdvertisingReport = 0x22,                  // 7.7.65.34
    SubrateChange = 0x23,                             // 7.7.65.35
    PeriodicAdvertisingSubeventDataRequest = 0x27,    // 7.7.65.36
    PeriodicAdvertisingResponseReport = 0x28,         // 7.7.65.37
    ReadAllRemoteFeaturesComplete = 0x2B,             // 7.7.65.38
    CSReadRemoteSupportedCapabilitiesComplete = 0x2C, // 7.7.65.39
    CSReadRemoteFAETableComplete = 0x2D,              // 7.7.65.40
    CSSecurityEnableComplete = 0x2E,                  // 7.7.65.41
    CSConfigComplete = 0x2F,                          // 7.7.65.42
    CSProcedureEnableComplete = 0x30,                 // 7.7.65.43
    CSSubeventResult = 0x31,                          // 7.7.65.44
    CSSubeventResultContinue = 0x32,                  // 7.7.65.45
    CSTestEndComplete = 0x33,                         // 7.7.65.46
    MonitoredAdvertisersReport = 0x34,                // 7.7.65.47
    FrameSpaceUpdateComplete = 0x35,                  // 7.7.65.48
}

#[derive(Debug)]
pub enum HCIEvent<'p> {
    DisconnectionComplete(DisconnectionCompleteEvent), // 7.7.5
    CommandComplete(CommandCompleteEvent<'p>),         // 7.7.14
    LEMetaEvent(LEMetaEvent<'p>),                      // 7.7.65
}

#[derive(Debug)]
pub enum HciParseError<'p> {
    InvalidField {
        field: &'p str,
        position: usize,
    },
    OutOfBounds {
        field: &'p str,
        position: usize,
    },
    InvalidLength {
        field: &'p str,
        expected: usize,
        found: usize,
    },
    NotImplemented {
        evcode: u8,
        sub_evcode: Option<u8>,
    },
}

impl<'p> HCIEvent<'p> {
    pub fn from_packet(packet: &'p HCIEventPacket) -> Result<HCIEvent<'p>, HciParseError<'p>> {
        let mut reader = Reader::new(packet.parameters);

        Ok(match packet.evcode.into() {
            HCIEventCode::DisconnectionComplete => {
                HCIEvent::DisconnectionComplete(DisconnectionCompleteEvent {
                    status: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                        field: "status",
                        position: reader.pos,
                    })?,
                    connection_handle: reader.read_u16().ok_or(HciParseError::OutOfBounds {
                        field: "connection_handle",
                        position: reader.pos,
                    })?,
                    reason: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                        field: "reason",
                        position: reader.pos,
                    })?,
                })
            }
            HCIEventCode::CommandComplete => HCIEvent::CommandComplete(CommandCompleteEvent {
                num_hci_command_packets: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                    field: "num_hci_command_packets",
                    position: reader.pos,
                })?,
                command_opcode: reader.read_u16().ok_or(HciParseError::OutOfBounds {
                    field: "command_opcode",
                    position: reader.pos,
                })?,
                return_parameters: reader.read_u8_slice(packet.len - reader.pos).ok_or(
                    HciParseError::OutOfBounds {
                        field: "return_parameters",
                        position: reader.pos,
                    },
                )?,
            }),
            HCIEventCode::LEMetaEvent => HCIEvent::LEMetaEvent(
                match reader
                    .read_u8()
                    .ok_or(HciParseError::OutOfBounds {
                        field: "sub_event_code",
                        position: reader.pos,
                    })?
                    .into()
                {
                    SubeventCode::ConnectionComplete => {
                        LEMetaEvent::ConnectionComplete(ConnectionCompleteEvent {
                            status: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                                field: "status",
                                position: reader.pos,
                            })?,
                            connection_handle: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "connection_handle",
                                    position: reader.pos,
                                },
                            )?,
                            role: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                                field: "role",
                                position: reader.pos,
                            })?,
                            peer_address_type: reader.read_u8().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "peer_address_type",
                                    position: reader.pos,
                                },
                            )?,
                            peer_address: reader.read_u8_slice(6).ok_or(
                                HciParseError::OutOfBounds {
                                    field: "peer_address",
                                    position: reader.pos,
                                },
                            )?,
                            connection_interval: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "connection_interval",
                                    position: reader.pos,
                                },
                            )?,
                            peripheral_latency: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "peripheral_latency",
                                    position: reader.pos,
                                },
                            )?,
                            supervision_timeout: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "supervision_timeout",
                                    position: reader.pos,
                                },
                            )?,
                            central_clock_accuracy: reader.read_u8().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "central_clock_accuracy",
                                    position: reader.pos,
                                },
                            )?,
                        })
                    }

                    SubeventCode::AdvertisingReport => {
                        LEMetaEvent::AdvertisingReport(AdvertisingReportIterator {
                            num_reports: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                                field: "num_reports",
                                position: reader.pos,
                            })?,
                            reader: Reader::new(
                                reader.read_u8_slice(packet.len - reader.pos).ok_or(
                                    HciParseError::OutOfBounds {
                                        field: "reports",
                                        position: reader.pos,
                                    },
                                )?,
                            ),
                        })
                    }
                    SubeventCode::ConnectionUpdateComplete => {
                        LEMetaEvent::ConnectionUpdateComplete(ConnectionUpdateCompleteEvent {
                            status: reader.read_u8().ok_or(HciParseError::OutOfBounds {
                                field: "status",
                                position: reader.pos,
                            })?,
                            connection_handle: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "connection_handle",
                                    position: reader.pos,
                                },
                            )?,
                            connection_interval: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "connection_interval",
                                    position: reader.pos,
                                },
                            )?,
                            peripheral_latency: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "peripheral_latency",
                                    position: reader.pos,
                                },
                            )?,
                            supervision_timeout: reader.read_u16().ok_or(
                                HciParseError::OutOfBounds {
                                    field: "supervision_timeout",
                                    position: reader.pos,
                                },
                            )?,
                        })
                    }
                    code => {
                        log::warn!("{:?} is not implemented skipping", code);

                        return Err(HciParseError::NotImplemented {
                            evcode: HCIEventCode::LEMetaEvent.into(),
                            sub_evcode: Some(code.into()),
                        });
                    }
                },
            ),
        })
    }
}

#[derive(Debug)]
pub struct DisconnectionCompleteEvent {
    pub status: u8,
    pub connection_handle: u16,
    pub reason: u8, // Bluetooth Core Spec 6.0 | [Vol 1] Part F | page 410
}

#[derive(Debug)]
pub struct CommandCompleteEvent<'p> {
    pub num_hci_command_packets: u8,
    pub command_opcode: u16,
    pub return_parameters: &'p [u8],
}

#[derive(Debug)]
pub enum LEMetaEvent<'p> {
    ConnectionComplete(ConnectionCompleteEvent<'p>), // 7.7.65.1
    AdvertisingReport(AdvertisingReportIterator<'p>), // 7.7.65.2
    ConnectionUpdateComplete(ConnectionUpdateCompleteEvent), // 7.7.65.3
    ReadAllRemoteFeaturesComplete(&'p [u8]),         // 7.7.65.38
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7.65.1 | page 2324
#[derive(Debug)]
pub struct ConnectionCompleteEvent<'p> {
    pub status: u8,
    pub connection_handle: u16,
    pub role: u8,
    pub peer_address_type: u8,
    pub peer_address: &'p [u8],
    pub connection_interval: u16,
    pub peripheral_latency: u16,
    pub supervision_timeout: u16,
    pub central_clock_accuracy: u8,
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7.65.2 | page 2327
#[derive(Debug)]
pub struct AdvertisingReport<'p> {
    pub event_type: u8,
    pub address_type: u8,
    pub address: &'p [u8],
    pub data: AdvertisingDataIterator<'p>,
    pub rssi: i8,
}

// Bluetooth Core spec 6.0 | [Vol 4] Part E, Section 7.7.65.3 | page 2330
#[derive(Debug)]
pub struct ConnectionUpdateCompleteEvent {
    pub status: u8,
    pub connection_handle: u16,
    pub connection_interval: u16,
    pub peripheral_latency: u16,
    pub supervision_timeout: u16,
}

#[derive(Debug)]
pub struct AdvertisingReportIterator<'p> {
    pub num_reports: u8,
    pub reader: Reader<'p>,
}

impl<'p> Iterator for AdvertisingReportIterator<'p> {
    type Item = AdvertisingReport<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.remaining() == 0 {
            return None;
        }

        Some(AdvertisingReport {
            event_type: self.reader.read_u8()?,
            address_type: self.reader.read_u8()?,
            address: self.reader.read_u8_slice(6)?,
            data: {
                let len = self.reader.read_u8()? as usize;
                AdvertisingDataIterator {
                    reader: Reader::new(self.reader.read_u8_slice(len)?),
                }
            },
            rssi: self.reader.read_u8()? as i8,
        })
    }
}

#[derive(Debug)]
pub struct AdvertisingDataIterator<'p> {
    pub reader: Reader<'p>,
}

impl<'p> Iterator for AdvertisingDataIterator<'p> {
    type Item = AdvertisingData<'p>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.remaining() == 0 {
            return None;
        }

        let len = self.reader.read_u8()? as usize;
        let ad_type = self.reader.read_u8()?.into();
        let data = self.reader.read_u8_slice(len - size_of::<u8>())?;
        let mut reader = Reader::new(data);

        match ad_type {
            AdvertisingDataType::Flags => Some(AdvertisingData::Flags(reader.read_u8()?)),
            AdvertisingDataType::IncompleteListOf16BitServiceUUIDs => Some(
                AdvertisingData::IncompleteListOf16BitServiceUUIDs(reader.read_u16_slice(len)?),
            ),
            AdvertisingDataType::CompleteListOf16BitServiceUUIDs => Some(
                AdvertisingData::CompleteListOf16BitServiceUUIDs(reader.read_u16_slice(len)?),
            ),
            AdvertisingDataType::IncompleteListOf32BitServiceUUIDs => Some(
                AdvertisingData::IncompleteListOf32BitServiceUUIDs(reader.read_u32_slice(len)?),
            ),
            AdvertisingDataType::CompleteListOf32BitServiceUUIDs => Some(
                AdvertisingData::CompleteListOf32BitServiceUUIDs(reader.read_u32_slice(len)?),
            ),
            AdvertisingDataType::IncompleteListOf128BitServiceUUIDs => Some(
                AdvertisingData::IncompleteListOf128BitServiceUUIDs(reader.read_u128_slice(len)?),
            ),
            AdvertisingDataType::CompleteListOf128BitServiceUUIDs => Some(
                AdvertisingData::CompleteListOf128BitServiceUUIDs(reader.read_u128_slice(len)?),
            ),
            AdvertisingDataType::ShortenedLocalName => Some(AdvertisingData::ShortenedLocalName(
                core::str::from_utf8(reader.read_u8_slice(reader.remaining())?).ok()?,
            )),
            AdvertisingDataType::CompleteLocalName => Some(AdvertisingData::CompleteLocalName(
                core::str::from_utf8(reader.read_u8_slice(reader.remaining())?).ok()?,
            )),
            AdvertisingDataType::TxPowerLevel => {
                Some(AdvertisingData::TxPowerLevel(reader.read_u8()? as i8))
            }
            AdvertisingDataType::ClassOfDevice => {
                Some(AdvertisingData::ClassOfDevice(reader.read_u32()?))
            }
            AdvertisingDataType::PeripheralConnectionIntervalRange => {
                Some(AdvertisingData::PeripheralConnectionIntervalRange(
                    reader.read_u8_slice(reader.remaining())?,
                ))
            }
            AdvertisingDataType::ServiceData => Some(AdvertisingData::ServiceData(
                reader.read_u8_slice(reader.remaining())?,
            )),
            AdvertisingDataType::Appearance => {
                Some(AdvertisingData::Appearance(reader.read_u16()?))
            }
            AdvertisingDataType::LEBluetoothDeviceAddress => {
                Some(AdvertisingData::LEBluetoothDeviceAddress(
                    reader.read_u8_slice(reader.remaining())?,
                ))
            }
            AdvertisingDataType::ManufacturerSpecificData => {
                Some(AdvertisingData::ManufacturerSpecificData(
                    reader.read_u8_slice(reader.remaining())?,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_disconnection_complete_event() {
        let packet = HCIEventPacket {
            evcode: HCIEventCode::DisconnectionComplete.into(),
            len: 4,
            parameters: &[0x00, 0x01, 0x00, 0x00],
        };

        let event = HCIEvent::from_packet(&packet);

        assert!(event.is_ok());

        log::info!("{:?}", event);

        let event = event.unwrap();

        if let HCIEvent::DisconnectionComplete(event) = event {
            assert_eq!(event.status, 0x00);
            assert_eq!(event.connection_handle, 0x0001);
            assert_eq!(event.reason, 0x13);
        } else {
            panic!("Unexpected event type");
        }
    }

    #[test]
    fn test_invalid_disconnection_complete_event() {
        let packet = HCIEventPacket {
            evcode: HCIEventCode::DisconnectionComplete.into(),
            len: 3,                          // Incorrect length
            parameters: &[0x00, 0x01, 0x00], // Missing Reason field
        };

        assert!(HCIEvent::from_packet(&packet).is_err());
    }
}
