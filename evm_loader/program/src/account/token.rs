use crate::error::{Error, Result};
use solana_program::account_info::AccountInfo;
use solana_program::program_pack::{IsInitialized, Pack};
use spl_token_2022::extension::StateWithExtensions;
use std::ops::Deref;
pub struct Account<'a, T: Pack + IsInitialized> {
    pub info: &'a AccountInfo<'a>,
    data: T,
}

impl<'a, T: Pack + IsInitialized> Account<'a, T> {
    pub fn from_account(info: &'a AccountInfo<'a>) -> Result<Self> {
        if !spl_token_2022::check_id(info.owner) {
            return Err(Error::AccountInvalidOwner2(
                *info.key,
                spl_token_2022::ID,
                *info.owner,
            ));
        }

        let data = info.try_borrow_data()?;
        let data = T::unpack(&data)?;

        Ok(Self { info, data })
    }

    pub fn into_data(self) -> T {
        self.data
    }
}

impl<'a, T: Pack + IsInitialized> Deref for Account<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct AccountState<'a> {
    pub info: &'a AccountInfo<'a>,
    data: spl_token_2022::state::Account,
}

impl<'a> AccountState<'a> {
    pub fn from_account(info: &'a AccountInfo<'a>) -> Result<Self> {
        if !spl_token_2022::check_id(info.owner) {
            return Err(Error::AccountInvalidOwner2(
                *info.key,
                spl_token_2022::ID,
                *info.owner,
            ));
        }
        let data = info.try_borrow_data()?;
        //let data = T::unpack(&data)?;
        let data_with_extensions: StateWithExtensions<spl_token_2022::state::Account> =
            StateWithExtensions::unpack(data.as_ref())
                .map_err(|_| Error::AccountMissing(*info.key))?;

        if !data_with_extensions.base.is_initialized() {
            return Err(Error::AccountMissing(*info.key));
        }
        let data: spl_token_2022::state::Account = data_with_extensions.base;

        Ok(Self { info, data })
    }
    
    #[must_use]
    pub fn into_data(self) -> spl_token_2022::state::Account {
        self.data
    }
}

impl<'a> Deref for AccountState<'a> {
    type Target = spl_token_2022::state::Account;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct AccountMint<'a> {
    pub info: &'a AccountInfo<'a>,
    data: spl_token_2022::state::Mint,
}

impl<'a> AccountMint<'a> {
    pub fn from_account(info: &'a AccountInfo<'a>) -> Result<Self> {
        if !spl_token_2022::check_id(info.owner) {
            return Err(Error::AccountInvalidOwner2(
                *info.key,
                spl_token_2022::ID,
                *info.owner,
            ));
        }
        let data = info.try_borrow_data()?;
        //let data = T::unpack(&data)?;
        let data_with_extensions: StateWithExtensions<spl_token_2022::state::Mint> =
            StateWithExtensions::unpack(data.as_ref())
                .map_err(|_| Error::AccountMissing(*info.key))?;

        if !data_with_extensions.base.is_initialized() {
            return Err(Error::AccountMissing(*info.key));
        }
        let data: spl_token_2022::state::Mint = data_with_extensions.base;

        Ok(Self { info, data })
    }

    #[must_use]
    pub fn into_data(self) -> spl_token_2022::state::Mint {
        self.data
    }
}

impl<'a> Deref for AccountMint<'a> {
    type Target = spl_token_2022::state::Mint;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub type State<'a> = AccountState<'a>;
pub type Mint<'a> = AccountMint<'a>;
