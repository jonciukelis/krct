use serde::Deserialize;
use serde::de::Deserializer;


#[derive(Deserialize)]
pub struct Row {
    #[serde(alias = "type")] 
    pub tx_type: Type,
    pub client: u16,
    pub tx: u32,
    pub amount: Amount
}

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback
}

pub enum Amount {
    None,
    Some(f64)
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
        {
            let s = String::deserialize(deserializer)?;
            Ok(match s.as_str() {
                "deposit" => Type::Deposit,
                "withdrawal" => Type::Withdrawal,
                "dispute" => Type::Dispute,
                "resolve" => Type::Resolve,
                "chargeback" => Type::Chargeback,
                _ => panic!("Unexpected row type: {:?}", s.as_str()),
            })
        }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
        {
            let amount: Amount = match f64::deserialize(deserializer) {
                Ok(amount) => Amount::Some(amount),
                Err(_) => Amount::None
            };
            Ok(amount)
        }
}