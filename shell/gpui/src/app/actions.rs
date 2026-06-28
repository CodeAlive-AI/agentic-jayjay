use gpui::actions;

actions!(
    jayjay,
    [
        OpenSettings,
        OpenAbout,
        // cmd-w: close the focused window (after dismissing any open overlay).
        CloseWindow,
        // escape: dismiss an open overlay without closing the window.
        Dismiss,
        Refresh,
        OpenRepository,
        OpenCommandPalette,
        OpenFind,
        CopyDiffSelection,
        OpenUserGuide,
        OpenJujutsuDocumentation,
        ReportIssue,
        OpenBookmarkManager,
        OpenOperationLog,
        OpenRepoInEditor,
        OpenRepoInTerminal,
        ShowRepoInFileManager,
        OpenRemoteRepository,
        GitFetchOrigin,
        GitPushDefault,
        ForgetStaleBookmarks,
        ToggleSideBySideDiff,
        ToggleIgnoreWhitespace,
        ToggleHideGitLfsFiles,
        ToggleTreeFileList,
        ZoomIn,
        ZoomOut,
        ResetZoom,
        Quit
    ]
);
