pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {
    pub fn AssociateNetworkAccountAsync<P0>(store: P0, networkname: &windows_core::HSTRING, networkaccountid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AssociateNetworkAccountAsync)(windows_core::Interface::as_raw(this), store.param().abi(), core::mem::transmute_copy(networkname), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportVcardToSystemAsync<P0>(stream: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportVcardToSystemAsync)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AssociateSocialNetworkAccountAsync<P0>(store: P0, networkname: &windows_core::HSTRING, networkaccountid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AssociateSocialNetworkAccountAsync)(windows_core::Interface::as_raw(this), store.param().abi(), core::mem::transmute_copy(networkname), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
windows_core::imp::define_interface!(IContactPartnerProvisioningManagerStatics, IContactPartnerProvisioningManagerStatics_Vtbl, 0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
impl windows_core::RuntimeType for IContactPartnerProvisioningManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportVcardToSystemAsync: usize,
}
windows_core::imp::define_interface!(IContactPartnerProvisioningManagerStatics2, IContactPartnerProvisioningManagerStatics2_Vtbl, 0xc26155f7_55ed_475d_9334_c5d484c30f1a);
impl windows_core::RuntimeType for IContactPartnerProvisioningManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessagePartnerProvisioningManagerStatics, IMessagePartnerProvisioningManagerStatics_Vtbl, 0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
impl windows_core::RuntimeType for IMessagePartnerProvisioningManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {
    pub fn ImportSmsToSystemAsync<P4>(incoming: bool, read: bool, body: &windows_core::HSTRING, sender: &windows_core::HSTRING, recipients: P4, deliverytime: super::super::super::Foundation::DateTime) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P4: windows_core::Param<windows_collections::IVectorView<windows_core::HSTRING>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportSmsToSystemAsync)(windows_core::Interface::as_raw(this), incoming, read, core::mem::transmute_copy(body), core::mem::transmute_copy(sender), recipients.param().abi(), deliverytime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ImportMmsToSystemAsync<P4, P6>(incoming: bool, read: bool, subject: &windows_core::HSTRING, sender: &windows_core::HSTRING, recipients: P4, deliverytime: super::super::super::Foundation::DateTime, attachments: P6) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P4: windows_core::Param<windows_collections::IVectorView<windows_core::HSTRING>>,
        P6: windows_core::Param<windows_collections::IVectorView<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportMmsToSystemAsync)(windows_core::Interface::as_raw(this), incoming, read, core::mem::transmute_copy(subject), core::mem::transmute_copy(sender), recipients.param().abi(), deliverytime, attachments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
