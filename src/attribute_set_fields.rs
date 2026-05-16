use serde::{Deserialize, Serialize};

macro_rules! attribute_field_enum {
    ($name:ident { $($variant:ident => $raw:literal),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $raw),*
                }
            }
        }
    };
}

attribute_field_enum!(AttributeStringField {
    DisplayName => "displayName",
    Path => "path",
    RelatedUniqueIdentifier => "relatedUniqueIdentifier",
    WeakRelatedUniqueIdentifier => "weakRelatedUniqueIdentifier",
    ContentType => "contentType",
    Title => "title",
    Version => "version",
    DomainIdentifier => "domainIdentifier",
    ContainerTitle => "containerTitle",
    ContainerDisplayName => "containerDisplayName",
    ContainerIdentifier => "containerIdentifier",
    Subject => "subject",
    Theme => "theme",
    ContentDescription => "contentDescription",
    Identifier => "identifier",
    SecurityMethod => "securityMethod",
    Creator => "creator",
    Kind => "kind",
    AccountIdentifier => "accountIdentifier",
    TextContent => "textContent",
    TranscribedTextContent => "transcribedTextContent",
    Comment => "comment",
    Copyright => "copyright",
    Role => "role",
    Rights => "rights",
    RatingDescription => "ratingDescription",
    Information => "information",
    Director => "director",
    Producer => "producer",
    Genre => "genre",
    OriginalFormat => "originalFormat",
    OriginalSource => "originalSource",
    KeySignature => "keySignature",
    TimeSignature => "timeSignature",
    AudioEncodingApplication => "audioEncodingApplication",
    Composer => "composer",
    Lyricist => "lyricist",
    Album => "album",
    Artist => "artist",
    MusicalGenre => "musicalGenre",
    MusicalInstrumentCategory => "musicalInstrumentCategory",
    MusicalInstrumentName => "musicalInstrumentName",
    ColorSpace => "colorSpace",
    AcquisitionMake => "acquisitionMake",
    AcquisitionModel => "acquisitionModel",
    CameraOwner => "cameraOwner",
    LensModel => "lensModel",
    ProfileName => "profileName",
    EXIFVersion => "EXIFVersion",
    EXIFGPSVersion => "EXIFGPSVersion",
    MeteringMode => "meteringMode",
    ExposureProgram => "exposureProgram",
    ExposureTimeString => "exposureTimeString",
    Headline => "headline",
    Instructions => "instructions",
    Thoroughfare => "thoroughfare",
    SubThoroughfare => "subThoroughfare",
    PostalCode => "postalCode",
    City => "city",
    StateOrProvince => "stateOrProvince",
    Country => "country",
    FullyFormattedAddress => "fullyFormattedAddress",
    NamedLocation => "namedLocation",
    GPSStatus => "GPSStatus",
    GPSMeasureMode => "GPSMeasureMode",
    GPSMapDatum => "GPSMapDatum",
    GPSProcessingMethod => "GPSProcessingMethod",
    GPSAreaInformation => "GPSAreaInformation",
});

attribute_field_enum!(AttributeReadOnlyStringField {
    TextContentSummary => "textContentSummary",
});

attribute_field_enum!(AttributeStringArrayField {
    AlternateNames => "alternateNames",
    ContentTypeTree => "contentTypeTree",
    Keywords => "keywords",
    ProviderDataTypeIdentifiers => "providerDataTypeIdentifiers",
    ProviderFileTypeIdentifiers => "providerFileTypeIdentifiers",
    ProviderInPlaceFileTypeIdentifiers => "providerInPlaceFileTypeIdentifiers",
    Audiences => "audiences",
    EncodingApplications => "encodingApplications",
    FontNames => "fontNames",
    AccountHandles => "accountHandles",
    MailboxIdentifiers => "mailboxIdentifiers",
    AuthorNames => "authorNames",
    RecipientNames => "recipientNames",
    AuthorEmailAddresses => "authorEmailAddresses",
    RecipientEmailAddresses => "recipientEmailAddresses",
    AuthorAddresses => "authorAddresses",
    RecipientAddresses => "recipientAddresses",
    PhoneNumbers => "phoneNumbers",
    EmailAddresses => "emailAddresses",
    InstantMessageAddresses => "instantMessageAddresses",
    Editors => "editors",
    Participants => "participants",
    Projects => "projects",
    ContentSources => "contentSources",
    ContactKeywords => "contactKeywords",
    Codecs => "codecs",
    MediaTypes => "mediaTypes",
    Organizations => "organizations",
    Languages => "languages",
    Publishers => "publishers",
    Contributors => "contributors",
    Coverage => "coverage",
    Performers => "performers",
    LayerNames => "layerNames",
});

attribute_field_enum!(AttributeNumberField {
    UserCreated => "userCreated",
    UserOwned => "userOwned",
    UserCurated => "userCurated",
    RankingHint => "rankingHint",
    SupportsPhoneCall => "supportsPhoneCall",
    SupportsNavigation => "supportsNavigation",
    ContainerOrder => "containerOrder",
    FileSize => "fileSize",
    PageCount => "pageCount",
    PageWidth => "pageWidth",
    PageHeight => "pageHeight",
    LikelyJunk => "likelyJunk",
    Duration => "duration",
    Streamable => "streamable",
    TotalBitRate => "totalBitRate",
    VideoBitRate => "videoBitRate",
    AudioBitRate => "audioBitRate",
    DeliveryType => "deliveryType",
    Rating => "rating",
    PlayCount => "playCount",
    Local => "local",
    ContentRating => "contentRating",
    AudioSampleRate => "audioSampleRate",
    AudioChannelCount => "audioChannelCount",
    Tempo => "tempo",
    AudioTrackNumber => "audioTrackNumber",
    GeneralMIDISequence => "generalMIDISequence",
    PixelHeight => "pixelHeight",
    PixelWidth => "pixelWidth",
    PixelCount => "pixelCount",
    BitsPerSample => "bitsPerSample",
    FlashOn => "flashOn",
    FocalLength => "focalLength",
    FocalLength35mm => "focalLength35mm",
    ISOSpeed => "ISOSpeed",
    Orientation => "orientation",
    WhiteBalance => "whiteBalance",
    Aperture => "aperture",
    ResolutionWidthDPI => "resolutionWidthDPI",
    ResolutionHeightDPI => "resolutionHeightDPI",
    ExposureMode => "exposureMode",
    ExposureTime => "exposureTime",
    HasAlphaChannel => "hasAlphaChannel",
    RedEyeOn => "redEyeOn",
    MaxAperture => "maxAperture",
    FNumber => "fNumber",
    Altitude => "altitude",
    Latitude => "latitude",
    Longitude => "longitude",
    Speed => "speed",
    ImageDirection => "imageDirection",
    GPSTrack => "GPSTrack",
    GPSDOP => "GPSDOP",
    GPSDestLatitude => "GPSDestLatitude",
    GPSDestLongitude => "GPSDestLongitude",
    GPSDestBearing => "GPSDestBearing",
    GPSDestDistance => "GPSDestDistance",
    GPSDifferental => "GPSDifferental",
    AllDay => "allDay",
});

attribute_field_enum!(AttributeReadOnlyNumberField {
    IsPriority => "isPriority",
});

attribute_field_enum!(AttributeUrlField {
    ContentURL => "contentURL",
    ThumbnailURL => "thumbnailURL",
    DarkThumbnailURL => "darkThumbnailURL",
    URL => "URL",
});

attribute_field_enum!(AttributeDataField {
    ThumbnailData => "thumbnailData",
    HTMLContentData => "HTMLContentData",
});

attribute_field_enum!(AttributeDateField {
    MetadataModificationDate => "metadataModificationDate",
    DownloadedDate => "downloadedDate",
    LastUsedDate => "lastUsedDate",
    ContentCreationDate => "contentCreationDate",
    ContentModificationDate => "contentModificationDate",
    AddedDate => "addedDate",
    RecordingDate => "recordingDate",
    Timestamp => "timestamp",
    GPSDateStamp => "GPSDateStamp",
    DueDate => "dueDate",
    CompletionDate => "completionDate",
    StartDate => "startDate",
    EndDate => "endDate",
});

attribute_field_enum!(AttributeDateArrayField {
    ImportantDates => "importantDates",
});

attribute_field_enum!(AttributePersonArrayField {
    Authors => "authors",
    PrimaryRecipients => "primaryRecipients",
    AdditionalRecipients => "additionalRecipients",
    HiddenAdditionalRecipients => "hiddenAdditionalRecipients",
});

attribute_field_enum!(AttributeStringArrayMapField {
    EmailHeaders => "emailHeaders",
});

