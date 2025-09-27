/// Holds the state
#[derive(Debug)]
pub struct App {
    /// If an image is in the process of being uploaded (but hasn't yet)
    pub is_uploading_image: bool,
    /// When the application was launched
    pub time_started: Instant,
    /// How long has passed since starting ferrishot
    pub time_elapsed: Duration,
    /// Config of the app
    pub config: Arc<Config>,
    /// A list of messages which obtained while the debug overlay is active
    pub logged_messages: Vec<Message>,
    /// How many selections were created throughout the
    /// lifetime of the App
    pub selections_created: usize,
    /// The full screenshot of the monitor from which ferrishot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    pub image: Arc<RgbaHandle>,
    /// Area of the screen that is selected for capture
    pub selection: Option<Selection>,
    /// Errors to display to the user
    pub errors: Errors,
    /// Whether to show an overlay with additional information (F12)
    pub show_debug_overlay: bool,
    /// Command line arguments passed
    pub cli: Arc<Cli>,

    /// Currently opened popup
    pub popup: Option<Popup>,
}