// Generated file, please don't edit manually.

impl Entry {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Entry = Entry::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_entry_type(&mut self) {
        self.entry_type = 0
    }
    #[inline]
    pub fn set_entry_type_(&mut self, v: EntryType) {
        self.entry_type = unsafe { ::std::mem::transmute::<EntryType, i32>(v) };
    }
    #[inline]
    pub fn get_entry_type(&self) -> EntryType {
        unsafe { ::std::mem::transmute::<i32, EntryType>(self.entry_type) }
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
    #[inline]
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    #[inline]
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    #[inline]
    pub fn get_index(&self) -> u64 {
        self.index
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = v;
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.data
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context.clear();
    }
    #[inline]
    pub fn set_context(&mut self, v: std::vec::Vec<u8>) {
        self.context = v;
    }
    #[inline]
    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.context
    }
    #[inline]
    pub fn take_context(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_sync_log(&mut self) {
        self.sync_log = false
    }
    #[inline]
    pub fn set_sync_log(&mut self, v: bool) {
        self.sync_log = v;
    }
    #[inline]
    pub fn get_sync_log(&self) -> bool {
        self.sync_log
    }
}
impl SnapshotMetadata {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotMetadata = SnapshotMetadata::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_conf_state(&self) -> bool {
        self.conf_state.is_some()
    }
    #[inline]
    pub fn clear_conf_state(&mut self) {
        self.conf_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_conf_state(&mut self, v: ConfState) {
        self.conf_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_conf_state(&self) -> &ConfState {
        match self.conf_state.as_ref() {
            Some(v) => v,
            None => ConfState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_conf_state(&mut self) -> &mut ConfState {
        if self.conf_state.is_none() {
            self.conf_state = ::std::option::Option::Some(ConfState::default());
        }
        self.conf_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_conf_state(&mut self) -> ConfState {
        self.conf_state.take().unwrap_or_else(ConfState::default)
    }
    #[inline]
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    #[inline]
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    #[inline]
    pub fn get_index(&self) -> u64 {
        self.index
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
}
impl Snapshot {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Snapshot = Snapshot::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = v;
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.data
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }
    #[inline]
    pub fn clear_metadata(&mut self) {
        self.metadata = ::std::option::Option::None
    }
    #[inline]
    pub fn set_metadata(&mut self, v: SnapshotMetadata) {
        self.metadata = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_metadata(&self) -> &SnapshotMetadata {
        match self.metadata.as_ref() {
            Some(v) => v,
            None => SnapshotMetadata::default_ref(),
        }
    }
    #[inline]
    pub fn mut_metadata(&mut self) -> &mut SnapshotMetadata {
        if self.metadata.is_none() {
            self.metadata = ::std::option::Option::Some(SnapshotMetadata::default());
        }
        self.metadata.as_mut().unwrap()
    }
    #[inline]
    pub fn take_metadata(&mut self) -> SnapshotMetadata {
        self.metadata
            .take()
            .unwrap_or_else(SnapshotMetadata::default)
    }
}
impl Message {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Message = Message::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_msg_type(&mut self) {
        self.msg_type = 0
    }
    #[inline]
    pub fn set_msg_type_(&mut self, v: MessageType) {
        self.msg_type = unsafe { ::std::mem::transmute::<MessageType, i32>(v) };
    }
    #[inline]
    pub fn get_msg_type(&self) -> MessageType {
        unsafe { ::std::mem::transmute::<i32, MessageType>(self.msg_type) }
    }
    #[inline]
    pub fn clear_to(&mut self) {
        self.to = 0
    }
    #[inline]
    pub fn set_to(&mut self, v: u64) {
        self.to = v;
    }
    #[inline]
    pub fn get_to(&self) -> u64 {
        self.to
    }
    #[inline]
    pub fn clear_from(&mut self) {
        self.from = 0
    }
    #[inline]
    pub fn set_from(&mut self, v: u64) {
        self.from = v;
    }
    #[inline]
    pub fn get_from(&self) -> u64 {
        self.from
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
    #[inline]
    pub fn clear_log_term(&mut self) {
        self.log_term = 0
    }
    #[inline]
    pub fn set_log_term(&mut self, v: u64) {
        self.log_term = v;
    }
    #[inline]
    pub fn get_log_term(&self) -> u64 {
        self.log_term
    }
    #[inline]
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    #[inline]
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    #[inline]
    pub fn get_index(&self) -> u64 {
        self.index
    }
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    #[inline]
    pub fn set_entries(&mut self, v: ::std::vec::Vec<Entry>) {
        self.entries = v;
    }
    #[inline]
    pub fn get_entries(&self) -> &::std::vec::Vec<Entry> {
        &self.entries
    }
    #[inline]
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<Entry> {
        &mut self.entries
    }
    #[inline]
    pub fn take_entries(&mut self) -> ::std::vec::Vec<Entry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    #[inline]
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    #[inline]
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
    #[inline]
    pub fn has_snapshot(&self) -> bool {
        self.snapshot.is_some()
    }
    #[inline]
    pub fn clear_snapshot(&mut self) {
        self.snapshot = ::std::option::Option::None
    }
    #[inline]
    pub fn set_snapshot(&mut self, v: Snapshot) {
        self.snapshot = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_snapshot(&self) -> &Snapshot {
        match self.snapshot.as_ref() {
            Some(v) => v,
            None => Snapshot::default_ref(),
        }
    }
    #[inline]
    pub fn mut_snapshot(&mut self) -> &mut Snapshot {
        if self.snapshot.is_none() {
            self.snapshot = ::std::option::Option::Some(Snapshot::default());
        }
        self.snapshot.as_mut().unwrap()
    }
    #[inline]
    pub fn take_snapshot(&mut self) -> Snapshot {
        self.snapshot.take().unwrap_or_else(Snapshot::default)
    }
    #[inline]
    pub fn clear_reject(&mut self) {
        self.reject = false
    }
    #[inline]
    pub fn set_reject(&mut self, v: bool) {
        self.reject = v;
    }
    #[inline]
    pub fn get_reject(&self) -> bool {
        self.reject
    }
    #[inline]
    pub fn clear_reject_hint(&mut self) {
        self.reject_hint = 0
    }
    #[inline]
    pub fn set_reject_hint(&mut self, v: u64) {
        self.reject_hint = v;
    }
    #[inline]
    pub fn get_reject_hint(&self) -> u64 {
        self.reject_hint
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context.clear();
    }
    #[inline]
    pub fn set_context(&mut self, v: std::vec::Vec<u8>) {
        self.context = v;
    }
    #[inline]
    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.context
    }
    #[inline]
    pub fn take_context(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
}
impl HardState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: HardState = HardState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
    #[inline]
    pub fn clear_vote(&mut self) {
        self.vote = 0
    }
    #[inline]
    pub fn set_vote(&mut self, v: u64) {
        self.vote = v;
    }
    #[inline]
    pub fn get_vote(&self) -> u64 {
        self.vote
    }
    #[inline]
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    #[inline]
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    #[inline]
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
}
impl ConfState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ConfState = ConfState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }
    #[inline]
    pub fn set_nodes(&mut self, v: ::std::vec::Vec<u64>) {
        self.nodes = v;
    }
    #[inline]
    pub fn get_nodes(&self) -> &::std::vec::Vec<u64> {
        &self.nodes
    }
    #[inline]
    pub fn mut_nodes(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.nodes
    }
    #[inline]
    pub fn take_nodes(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.nodes, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_learners(&mut self) {
        self.learners.clear();
    }
    #[inline]
    pub fn set_learners(&mut self, v: ::std::vec::Vec<u64>) {
        self.learners = v;
    }
    #[inline]
    pub fn get_learners(&self) -> &::std::vec::Vec<u64> {
        &self.learners
    }
    #[inline]
    pub fn mut_learners(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.learners
    }
    #[inline]
    pub fn take_learners(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.learners, ::std::vec::Vec::new())
    }
}
impl ConfChange {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ConfChange = ConfChange::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub fn clear_change_type(&mut self) {
        self.change_type = 0
    }
    #[inline]
    pub fn set_change_type_(&mut self, v: ConfChangeType) {
        self.change_type = unsafe { ::std::mem::transmute::<ConfChangeType, i32>(v) };
    }
    #[inline]
    pub fn get_change_type(&self) -> ConfChangeType {
        unsafe { ::std::mem::transmute::<i32, ConfChangeType>(self.change_type) }
    }
    #[inline]
    pub fn clear_node_id(&mut self) {
        self.node_id = 0
    }
    #[inline]
    pub fn set_node_id(&mut self, v: u64) {
        self.node_id = v;
    }
    #[inline]
    pub fn get_node_id(&self) -> u64 {
        self.node_id
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context.clear();
    }
    #[inline]
    pub fn set_context(&mut self, v: std::vec::Vec<u8>) {
        self.context = v;
    }
    #[inline]
    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.context
    }
    #[inline]
    pub fn take_context(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
}
impl EntryType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [EntryType] = &[EntryType::EntryNormal, EntryType::EntryConfChange];
        VALUES
    }
}
impl MessageType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [MessageType] = &[
            MessageType::MsgHup,
            MessageType::MsgBeat,
            MessageType::MsgPropose,
            MessageType::MsgAppend,
            MessageType::MsgAppendResponse,
            MessageType::MsgRequestVote,
            MessageType::MsgRequestVoteResponse,
            MessageType::MsgSnapshot,
            MessageType::MsgHeartbeat,
            MessageType::MsgHeartbeatResponse,
            MessageType::MsgUnreachable,
            MessageType::MsgSnapStatus,
            MessageType::MsgCheckQuorum,
            MessageType::MsgTransferLeader,
            MessageType::MsgTimeoutNow,
            MessageType::MsgReadIndex,
            MessageType::MsgReadIndexResp,
            MessageType::MsgRequestPreVote,
            MessageType::MsgRequestPreVoteResponse,
        ];
        VALUES
    }
}
impl ConfChangeType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ConfChangeType] = &[
            ConfChangeType::AddNode,
            ConfChangeType::RemoveNode,
            ConfChangeType::AddLearnerNode,
        ];
        VALUES
    }
}