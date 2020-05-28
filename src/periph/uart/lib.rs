//! Universal Asynchronous Receiver/Transmitter

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic UART peripheral variant.
    pub trait UartMap {}

    /// Generic UART peripheral.
    pub struct UartPeriph;

    UART {
        DR {
            0x20 RwReg;
            DATA { RwRwRegFieldBits }
            FE { RoRwRegFieldBit }
            PE { RoRwRegFieldBit }
            BE { RoRwRegFieldBit }
            OE { RoRwRegFieldBit }
        }
        RSR {
            0x20 RoReg;
            FE { RoRoRegFieldBit }
            PE { RoRoRegFieldBit }
            BE { RoRoRegFieldBit }
            OE { RoRoRegFieldBit }
        }
        ECR {
            0x20 WoReg;
            DATA { WoWoRegFieldBits }
        }
        FR {
            0x20 RoReg;
            CTS { RoRoRegFieldBit }
            BUSY { RoRoRegFieldBit }
            RXFE { RoRoRegFieldBit }
            TXFF { RoRoRegFieldBit }
            RXFF { RoRoRegFieldBit }
            TXFE { RoRoRegFieldBit }
        }
        ILPR {
            0x20 RwReg;
            ILPDVSR { RwRwRegFieldBits }
        }
        IBRD {
            0x20 RwReg;
            DIVINT { RwRwRegFieldBits }
        }
        FBRD {
            0x20 RwReg;
            DIVFRAC { RwRwRegFieldBits }
        }
        LCRH {
            0x20 RwReg;
            BRK { RwRwRegFieldBit }
            PEN { RwRwRegFieldBit }
            EPS { RwRwRegFieldBit }
            STP2 { RwRwRegFieldBit }
            FEN { RwRwRegFieldBit }
            WLEN { RwRwRegFieldBits }
            SPS { RwRwRegFieldBit }
        }
        CTL {
            0x20 RwReg;
            UARTEN { RwRwRegFieldBit }
            SIREN { RwRwRegFieldBit }
            SIRLP { RwRwRegFieldBit }
            EOT { RwRwRegFieldBit }
            HSE { RwRwRegFieldBit }
            LIN { RwRwRegFieldBit }
            LBE { RwRwRegFieldBit }
            TXE { RwRwRegFieldBit }
            RXE { RwRwRegFieldBit }
            RTSEN { RwRwRegFieldBit }
            CTSEN { RwRwRegFieldBit }
        }
        IFLS {
            0x20 RwReg;
            TXIFLSEL { RwRwRegFieldBits }
            RXIFLSEL { RwRwRegFieldBits }
        }
        IM {
            0x20 RwReg;
            RXIM { RwRwRegFieldBit }
            TXIM { RwRwRegFieldBit }
            RTIM { RwRwRegFieldBit }
            FEIM { RwRwRegFieldBit }
            PEIM { RwRwRegFieldBit }
            BEIM { RwRwRegFieldBit }
            OEIM { RwRwRegFieldBit }
            NINEBITIM { RwRwRegFieldBit }
            LMSBIM { RwRwRegFieldBit }
            LME1IM { RwRwRegFieldBit }
            LME5IM { RwRwRegFieldBit }
        }
        RIS {
            0x20 RoReg;
            RXRIS { RoRoRegFieldBit }
            TXRIS { RoRoRegFieldBit }
            RTRIS { RoRoRegFieldBit }
            FERIS { RoRoRegFieldBit }
            PERIS { RoRoRegFieldBit }
            BERIS { RoRoRegFieldBit }
            OERIS { RoRoRegFieldBit }
            NINEBITRIS { RoRoRegFieldBit }
            LMSBRIS { RoRoRegFieldBit }
            LME1RIS { RoRoRegFieldBit }
            LME5RIS { RoRoRegFieldBit }
        }
        MIS {
            0x20 RoReg;
            RXMIS { RoRoRegFieldBit }
            TXMIS { RoRoRegFieldBit }
            RTMIS { RoRoRegFieldBit }
            FEMIS { RoRoRegFieldBit }
            PEMIS { RoRoRegFieldBit }
            BEMIS { RoRoRegFieldBit }
            OEMIS { RoRoRegFieldBit }
            NINEBITMIS { RoRoRegFieldBit }
            LMSBMIS { RoRoRegFieldBit }
            LME1MIS { RoRoRegFieldBit }
            LME5MIS { RoRoRegFieldBit }

        }
        ICR {
            0x20 WoReg;
            RXIC { WoWoRegFieldBit }
            TXIC { WoWoRegFieldBit }
            RTIC { WoWoRegFieldBit }
            FEIC { WoWoRegFieldBit }
            PEIC { WoWoRegFieldBit }
            BEIC { WoWoRegFieldBit }
            OEIC { WoWoRegFieldBit }
            NINEBITIC { WoWoRegFieldBit }
            LMSBIC { WoWoRegFieldBit }
            LME1IC { WoWoRegFieldBit }
            LME5IC { WoWoRegFieldBit }
        }
        DMACTL {
            0x20 RwReg;
            RXDMAE { RwRwRegFieldBit }
            TXDMAE { RwRwRegFieldBit }
            DMAERR { RwRwRegFieldBit }
        }
        LCTL {
            0x20 RwReg;
            MASTER { RwRwRegFieldBit }
            BLEN { RwRwRegFieldBits }
        }
        LSS {
            0x20 RoReg;
            TSS { RoRoRegFieldBits }
        }
        LTIM {
            0x20 RoReg;
            TIMER { RoRoRegFieldBits }
        }
        NINEBITADDR {
            0x20 RwReg;
            ADDR { RwRwRegFieldBits }
            NINEBITEN { RwRwRegFieldBit }
        }
        NINEBITAMASK {
            0x20 RwReg;
            MASK { RwRwRegFieldBits }
            RANGE { RoRwRegFieldBits }
        }
        PP {
            0x20 RoReg;
            NB { RoRoRegFieldBit }
        }
        CC {
            0x20 RwReg;
            CS { RwRwRegFieldBits }
        }
    }
}

macro_rules! map_uart {
    ($uart_macro_doc:expr, $uart_macro:ident, $uart_ty_doc:expr, $uart_ty:ident, $uart:ident,) => {
        periph::map! {
            #[doc = $uart_macro_doc]
            pub macro $uart_macro;

            #[doc = $uart_ty_doc]
            pub struct $uart_ty;

            impl UartMap for $uart_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            UART {
                $uart;
                DR {
                    DR;
                    DATA { DATA }
                    FE { FE }
                    PE { PE }
                    BE { BE }
                    OE { OE }
                }
                RSR {
                    RSR;
                    FE { FE }
                    PE { PE }
                    BE { BE }
                    OE { OE }
                }
                ECR {
                    ECR;
                    DATA { DATA }
                }
                FR {
                    FR;
                    CTS { CTS }
                    BUSY { BUSY }
                    RXFE { RXFE }
                    TXFF { TXFF }
                    RXFF { RXFF }
                    TXFE { TXFE }
                }
                ILPR {
                    ILPR;
                    ILPDVSR { ILPDVSR }
                }
                IBRD {
                    IBRD;
                    DIVINT { DIVINT }
                }
                FBRD {
                    FBRD;
                    DIVFRAC { DIVFRAC }
                }
                LCRH {
                    LCRH;
                    BRK { BRK }
                    PEN { PEN }
                    EPS { EPS }
                    STP2 { STP2 }
                    FEN { FEN }
                    WLEN { WLEN }
                    SPS { SPS }
                }
                CTL {
                    CTL;
                    UARTEN { UARTEN }
                    SIREN { SIREN }
                    SIRLP { SIRLP }
                    EOT { EOT }
                    HSE { HSE }
                    LIN { LIN }
                    LBE { LBE }
                    TXE { TXE }
                    RXE { RXE }
                    RTSEN { RTSEN }
                    CTSEN { CTSEN }
                }
                IFLS {
                    IFLS;
                    TXIFLSEL { TXIFLSEL }
                    RXIFLSEL { RXIFLSEL }
                }
                IM {
                    IM;
                    RXIM { RXIM }
                    TXIM { TXIM }
                    RTIM { RTIM }
                    FEIM { FEIM }
                    PEIM { PEIM }
                    BEIM { BEIM }
                    OEIM { OEIM }
                    NINEBITIM { NINEBITIM }
                    LMSBIM { LMSBIM }
                    LME1IM { LME1IM }
                    LME5IM { LME5IM }
                }
                RIS {
                    RIS;
                    RXRIS { RXRIS }
                    TXRIS { TXRIS }
                    RTRIS { RTRIS }
                    FERIS { FERIS }
                    PERIS { PERIS }
                    BERIS { BERIS }
                    OERIS { OERIS }
                    NINEBITRIS { NINEBITRIS }
                    LMSBRIS { LMSBRIS }
                    LME1RIS { LME1RIS }
                    LME5RIS { LME5RIS }
                }
                MIS {
                    MIS;
                    RXMIS { RXMIS }
                    TXMIS { TXMIS }
                    RTMIS { RTMIS }
                    FEMIS { FEMIS }
                    PEMIS { PEMIS }
                    BEMIS { BEMIS }
                    OEMIS { OEMIS }
                    NINEBITMIS { NINEBITMIS }
                    LMSBMIS { LMSBMIS }
                    LME1MIS { LME1MIS }
                    LME5MIS { LME5MIS }

                }
                ICR {
                    ICR;
                    RXIC { RXIC }
                    TXIC { TXIC }
                    RTIC { RTIC }
                    FEIC { FEIC }
                    PEIC { PEIC }
                    BEIC { BEIC }
                    OEIC { OEIC }
                    NINEBITIC { NINEBITIC }
                    LMSBIC { LMSBIC }
                    LME1IC { LME1IC }
                    LME5IC { LME5IC }
                }
                DMACTL {
                    DMACTL;
                    RXDMAE { RXDMAE }
                    TXDMAE { TXDMAE }
                    DMAERR { DMAERR }
                }
                LCTL {
                    LCTL;
                    MASTER { MASTER }
                    BLEN { BLEN }
                }
                LSS {
                    LSS;
                    TSS { TSS }
                }
                LTIM {
                    LTIM;
                    TIMER { TIMER }
                }
                NINEBITADDR {
                    NINEBITADDR;
                    ADDR { ADDR }
                    NINEBITEN { NINEBITEN }
                }
                NINEBITAMASK {
                    NINEBITAMASK;
                    MASK { MASK }
                    RANGE { RANGE }
                }
                PP {
                    PP;
                    NB { NB }
                }
                CC {
                    CC;
                    CS { CS }
                }
            }
        }
    };
}

#[cfg(tisl_mcu = "cc2538")]
map_uart! {
    "Extracts UART0 register tokens.",
    periph_uart0,
    "UART0 peripheral variant.",
    Uart0,
    UART0,
}

#[cfg(tisl_mcu = "cc2538")]
map_uart! {
    "Extracts UART1 register tokens.",
    periph_uart1,
    "UART1 peripheral variant.",
    Uart1,
    UART1,
}