use std::{collections::HashMap, path::PathBuf};

use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use crate::{types::ConnectionFlags, Error};

/// A connection profile.
///
/// See [`Device::reapply()`](crate::device::Device::reapply()) for the difference between a
/// [`Connection`] and an [`AppliedConnection`].
#[derive(Clone, Debug)]
pub struct Connection {
    pub(crate) zbus: zbus::Connection,
    pub(crate) path: OwnedObjectPath,
}

crate::zproxy_pathed!(
    Connection,
    crate::raw::settings_connection::SettingsConnectionProxy<'_>
);

impl Connection {
    /// Update the connection with new settings and properties and save the connection to disk.
    ///
    /// This replaces all previous settings and properties.
    ///
    /// Secrets may be part of the update request, and will be either stored in persistent storage
    /// or sent to a Secret Agent for storage, depending on the flags associated with each secret.
    pub async fn update(
        &self,
        properties: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> Result<(), Error> {
        self.raw()
            .await?
            .update(properties)
            .await
            .map_err(Error::ZBus)
    }

    /// Update the connection with new settings and properties, but do not immediately save the connection to disk.
    ///
    /// This replaces all previous settings and properties.
    ///
    /// Secrets may be part of the update request and may sent to a Secret Agent for storage,
    /// depending on the flags associated with each secret.
    ///
    /// Use the [`Connection::save()`] method to save these changes to disk.
    ///
    /// Note that unsaved changes will be lost if the connection is reloaded from disk (either
    /// automatically on file change or due to an explicit `ReloadConnections` call).
    pub async fn update_in_memory(
        &self,
        properties: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> Result<(), Error> {
        self.raw()
            .await?
            .update_unsaved(properties)
            .await
            .map_err(Error::ZBus)
    }

    /// Delete the connection.
    pub async fn delete(&self) -> Result<(), Error> {
        self.raw().await?.delete().await.map_err(Error::ZBus)
    }

    /// Get the settings maps describing this network configuration.
    ///
    /// This will never include any secrets required for connection to the network, as those are
    /// often protected. Secrets must be requested separately using the `GetSecrets` call.
    pub async fn settings(&self) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        self.raw().await?.get_settings().await.map_err(Error::ZBus)
    }

    /// Get the secrets belonging to this network configuration.
    ///
    /// Only secrets from persistent storage or a Secret Agent running in the requestor's session
    /// will be returned. The user will never be prompted for secrets as a result of this request.
    pub async fn secrets(&self) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        self.secrets_for_setting("").await
    }

    /// Get the secrets belonging to this network configuration, for a particular setting.
    ///
    /// Only secrets from persistent storage or a Secret Agent running in the requestor's session
    /// will be returned. The user will never be prompted for secrets as a result of this request.
    pub async fn secrets_for_setting(
        &self,
        setting_name: &str,
    ) -> Result<HashMap<String, HashMap<String, OwnedValue>>, Error> {
        self.raw()
            .await?
            .get_secrets(setting_name)
            .await
            .map_err(Error::ZBus)
    }

    /// Clear the secrets belonging to this network connection profile.
    pub async fn clear_secrets(&self) -> Result<(), Error> {
        self.raw().await?.clear_secrets().await.map_err(Error::ZBus)
    }

    /// Save a connection previously updated with [`Connection::update_in_memory()`] to persistent storage.
    pub async fn save(&self) -> Result<(), Error> {
        self.raw().await?.save().await.map_err(Error::ZBus)
    }

    // TODO: Update2

    /// Indicates whether the in-memory state of the connection matches the on-disk state.
    ///
    /// This flag will be unset when [`Connection::update_in_memory()`] is called or when any
    /// connection details change, and set when the connection is saved to disk via
    /// [`Connection::save()`] or from internal operations.
    pub async fn is_saved(&self) -> Result<bool, Error> {
        self.raw()
            .await?
            .unsaved()
            .await
            .map(|flag| !flag)
            .map_err(Error::ZBus)
    }

    /// Additional flags of the connection profile.
    pub async fn flags(&self) -> Result<ConnectionFlags, Error> {
        let value = self.raw().await?.flags().await?;
        Ok(ConnectionFlags::from_bits_retain(value))
    }

    /// File that stores the connection when the connection is file-backed.
    pub async fn filename(&self) -> Result<PathBuf, Error> {
        self.raw()
            .await?
            .filename()
            .await
            .map(|path| path.into())
            .map_err(Error::ZBus)
    }
}
