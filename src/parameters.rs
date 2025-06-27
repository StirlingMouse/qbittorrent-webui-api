/// Torrent List/info parameter object
#[derive(Debug)]
pub struct TorrentListParams {
    /// Filter torrent list by state. Allowed state filters: TorrentState
    pub filter: Option<TorrentState>,
    /// Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category"). Remember to URL-encode the category name. For example, `My category` becomes `My%20category`
    pub category: Option<String>,
    /// Get torrents with the given tag (empty string means "without tag"; no "tag" parameter means "any tag". Remember to URL-encode the category name. For example, `My tag` becomes `My%20tag`
    pub tag: Option<String>,
    /// Sort torrents by given key. They can be sorted using any field of the response's JSON array (which are documented below) as the sort key.
    pub sort: Option<TorrentSort>,
    /// Enable reverse sorting. Defaults to `false`
    pub reverse: bool,
    /// Limit the number of torrents returned
    pub limit: Option<i64>,
    /// Set offset (if less than 0, offset from end)
    pub offset: Option<i64>,
    /// Filter by hashes. Can contain multiple hashes separated by `|`
    pub hashes: Option<Vec<String>>,
}

impl TorrentListParams {
    pub fn default() -> Self {
        Self {
            filter: None,
            category: None,
            tag: None,
            sort: None,
            reverse: false,
            limit: None,
            offset: None,
            hashes: None,
        }
    }
}

/// Posibel Torrent states
#[derive(Debug)]
pub enum TorrentState {
    All,
    Downloading,
    Seeding,
    Completed,
    Stopped,
    Active,
    Inactive,
    Running,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}

impl ToString for TorrentState {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("all"),
            Self::Downloading => String::from("downloading"),
            Self::Seeding => String::from("seeding"),
            Self::Completed => String::from("completed"),
            Self::Stopped => String::from("stopped"),
            Self::Active => String::from("active"),
            Self::Inactive => String::from("inactive"),
            Self::Running => String::from("running"),
            Self::Stalled => String::from("stalled"),
            Self::StalledUploading => String::from("stalled_uploading"),
            Self::StalledDownloading => String::from("stalled_downloading"),
            Self::Errored => String::from("errored"),
        }
    }
}

/// Torrent sort fields
#[derive(Debug)]
pub enum TorrentSort {
    /// Time when the torrent was added to the client
    AddedOn,
    /// Amount of data left to download
    AmountLeft,
    /// Whether this torrent is managed by Automatic Torrent Management
    AutoTmm,
    /// Percentage of file pieces currently available
    Availability,
    /// Category of the torrent
    Category,
    /// Amount of transfer data completed
    Completed,
    /// Time when the torrent completed
    CompletionOn,
    /// Torrent content path
    ContentPath,
    /// Torrent download speed limit.
    DlLimit,
    /// Torrent download speed
    Dlspeed,
    /// Amount of data downloaded
    Downloaded,
    /// Amount of data downloaded this session
    DownloadedSession,
    /// Torrent ETA
    Eta,
    /// First last piece are prioritized
    FLPiecePrio,
    /// Force start is enabled for this torrent
    ForceStart,
    /// Torrent hash
    Hash,
    /// True if torrent is from a private tracker
    Private,
    /// Last time when a chunk was downloaded/uploaded
    LastActivity,
    /// Magnet URI corresponding to this torrent
    MagnetUri,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    MaxRatio,
    /// Maximum seeding time until torrent is stopped from seeding
    MaxSeedingTime,
    /// Torrent name
    Name,
    /// Number of seeds in the swarm
    NumComplete,
    /// Number of leechers in the swarm
    NumIncomplete,
    /// Number of leechers connected to
    NumLeechs,
    /// Number of seeds connected to
    NumSeeds,
    /// Torrent priority
    Priority,
    /// Torrent progress
    Progress,
    /// Torrent share ratio.
    Ratio,
    RatioLimit,
    /// Time until the next tracker reannounce
    Reannounce,
    /// Path where this torrent's data is stored
    SavePath,
    /// Torrent elapsed time while complete
    SeedingTime,
    /// Torrent elapsed time while complete limit
    SeedingTimeLimit,
    /// Time when this torrent was last seen complete
    SeenComplete,
    /// True if sequential download is enabled
    SeqDl,
    /// Total size of files selected for download
    Size,
    /// Torrent state.
    State,
    /// Super seeding state
    SuperSeeding,
    /// Tag list of the torrent
    Tags,
    /// Total active time
    TimeActive,
    /// Total size of all file in this torrent. Including unselected ones
    TotalSize,
    /// The first tracker with working status. Empty string if no tracker is working.
    Tracker,
    /// Torrent upload speed limit
    UpLimit,
    /// Amount of data uploaded
    Uploaded,
    /// Amount of data uploaded this session
    UploadedSession,
    /// Torrent upload speed
    Upspeed,
}

impl ToString for TorrentSort {
    fn to_string(&self) -> String {
        match self {
            Self::AddedOn => String::from("added_on"),
            Self::AmountLeft => String::from("amount_left"),
            Self::AutoTmm => String::from("auto_tmm"),
            Self::Availability => String::from("availability"),
            Self::Category => String::from("category"),
            Self::Completed => String::from("completed"),
            Self::CompletionOn => String::from("completion_on"),
            Self::ContentPath => String::from("content_path"),
            Self::DlLimit => String::from("dl_limit"),
            Self::Dlspeed => String::from("dlspeed"),
            Self::Downloaded => String::from("downloaded"),
            Self::DownloadedSession => String::from("downloaded_session"),
            Self::Eta => String::from("eta"),
            Self::FLPiecePrio => String::from("f_l_piece_prio"),
            Self::ForceStart => String::from("force_start"),
            Self::Hash => String::from("hash"),
            Self::Private => String::from("private"),
            Self::LastActivity => String::from("last_activity"),
            Self::MagnetUri => String::from("magnet_uri"),
            Self::MaxRatio => String::from("max_ratio"),
            Self::MaxSeedingTime => String::from("max_seeding_time"),
            Self::Name => String::from("name"),
            Self::NumComplete => String::from("num_complete"),
            Self::NumIncomplete => String::from("num_incomplete"),
            Self::NumLeechs => String::from("num_leechs"),
            Self::NumSeeds => String::from("num_seeds"),
            Self::Priority => String::from("priority"),
            Self::Progress => String::from("progress"),
            Self::Ratio => String::from("ratio"),
            Self::RatioLimit => String::from("ratio_limit"),
            Self::Reannounce => String::from("reannounce"),
            Self::SavePath => String::from("save_path"),
            Self::SeedingTime => String::from("seeding_time"),
            Self::SeedingTimeLimit => String::from("seeding_time_limit"),
            Self::SeenComplete => String::from("seen_complete"),
            Self::SeqDl => String::from("seq_dl"),
            Self::Size => String::from("size"),
            Self::State => String::from("state"),
            Self::SuperSeeding => String::from("super_seeding"),
            Self::Tags => String::from("tags"),
            Self::TimeActive => String::from("time_active"),
            Self::TotalSize => String::from("total_size"),
            Self::Tracker => String::from("tracker"),
            Self::UpLimit => String::from("up_limit"),
            Self::Uploaded => String::from("uploaded"),
            Self::UploadedSession => String::from("uploaded_session"),
            Self::Upspeed => String::from("upspeed"),
        }
    }
}

/// Add torrent parameter object
#[derive(Debug)]
pub struct TorrentAddUrls {
    /// Torrent URLs separated with newlines
    pub urls: Vec<String>,
    // TODO: add `pub torrents`
    /// Download folder
    pub savepath: Option<String>,
    /// Category for the torrent
    pub category: Option<String>,
    /// Tags for the torrent, split by `,`
    pub tags: Option<Vec<String>>,
    /// Skip hash checking. Possible values are `true`, `false` (default)
    pub skip_checking: bool,
    /// Add torrents in the stopped state. Possible values are `true`, `false` (default)
    pub stopped: bool,
    /// Create the root folder. Possible values are `"true"`, `"false"`, unset (default)
    pub root_folder: Option<String>,
    /// Rename torrent
    pub rename: Option<String>,
    /// Set torrent upload speed limit. Unit in bytes/second
    pub up_limit: Option<i64>,
    /// Set torrent download speed limit. Unit in bytes/second
    pub dl_limit: Option<i64>,
    /// Set torrent share ratio limit
    pub ratio_limit: Option<f32>,
    /// Set torrent seeding time limit. Unit in minutes
    pub seeding_time_limit: Option<i64>,
    /// Whether Automatic Torrent Management should be used
    pub auto_tmm: bool,
    /// Enable sequential download. Possible values are `true`, `false` (default)
    pub sequential_download: bool,
    /// Prioritize download first last piece. Possible values are `true`, `false` (default)
    pub first_last_piece_prio: bool,
}

impl TorrentAddUrls {
    pub fn default(urls: Vec<String>) -> Self {
        Self {
            urls,
            savepath: None,
            category: None,
            tags: None,
            skip_checking: false,
            stopped: false,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            ratio_limit: None,
            seeding_time_limit: None,
            auto_tmm: false,
            sequential_download: false,
            first_last_piece_prio: false,
        }
    }
}
