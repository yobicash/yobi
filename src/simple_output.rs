use libyobicash::errors::*;
use libyobicash::amount::YAmount;
use libyobicash::crypto::elliptic::keys::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YAmountOutput {
    sk: YSecretKey,
    to: YPublicKey,
    amount: YAmount,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YDataOutput {
    sk: YSecretKey,
    to: YPublicKey,
    data_path: String,
}
