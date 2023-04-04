use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Copy, Clone)]
pub enum NinjaNames {
    FujibayashiNagato,
    MomochiSandayu,
    IshikawaGoemon,
    HattoriHanzo,
    MochizukiChiyome,
    FumaKotaro,
    JinichiKawakami,
}

impl Display for NinjaNames {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NinjaNames::FujibayashiNagato => {
                write!(formatter, "Fujibayashi Nagato")
            }
            NinjaNames::MomochiSandayu => {
                write!(formatter, "Momochi Sandayu")
            }
            NinjaNames::IshikawaGoemon => {
                write!(formatter, "Ishikawa Goemon")
            }
            NinjaNames::HattoriHanzo => {
                write!(formatter, "Hattori Hanzo")
            }
            NinjaNames::MochizukiChiyome => {
                write!(formatter, "Mochizuki Chiyome")
            }
            NinjaNames::FumaKotaro => {
                write!(formatter, "Fuma Kotaro")
            }
            NinjaNames::JinichiKawakami => {
                write!(formatter, "Jinichi Kawakami")
            }
        }
    }
}
