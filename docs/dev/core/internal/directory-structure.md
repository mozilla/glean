# Directory structure

This page describes the contents of the directories where Glean stores its data.

All Glean data is inside a single root directory with the name `glean_data`.

On Android, this directory lives inside the [`ApplicationInfo.dataDir`](https://developer.android.com/reference/android/content/pm/ApplicationInfo.html#dataDir) directory associated with the application.

On iOS, this directory lives inside the [`Documents`](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html) directory associated with the application.

For the Python bindings, if no directory is specified, it is stored in a temporary directory and cleared at exit.

Within the `glean_data` directory are the following contents:

- `db`: Contains the [rkv](https://github.com/mozilla/rkv) database used to persist ping and user lifetime metrics.

- `events`: Contains flat files containing persisted events before they are collected into pings.

- `pending_pings`: Pings are written here before they are picked up by the ping uploader to send to the submission endpoint.

- `tmp`: Pings are written here and then moved to the `pending_pings` directory when finished to make sure that partially-written pings to not get queued for sending.  
  (The standard system temporary directory is not used for this because it is not guaranteed to be on the same volume as the `glean_data` directory on Android).
