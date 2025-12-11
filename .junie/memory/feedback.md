[2025-12-10 12:57] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "scope and structure",
"EXPECTATION": "The VFS trait must be a project-wide IO abstraction: all existing file reads/listings should go through
it, not only LSP methods; also split Vfs, DefaultFs, and RemoteFs into separate files.",
"NEW INSTRUCTION": "WHEN modifying filesystem codebase THEN route all file IO via the Vfs trait"
}

[2025-12-10 13:14] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "module structure and API",
"EXPECTATION": "Keep VFS in a single file with hidden implementations and add a replaceFS(fsName: string, args: Map<
String,String>) method to swap filesystem at runtime.",
"NEW INSTRUCTION": "WHEN editing VFS module or API THEN maintain single vfs.rs; make impls private; include replaceFS(
name,args)"
}

[2025-12-10 13:22] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "module structure and API",
"EXPECTATION": "Keep VFS in a single file with hidden implementations and provide replaceFS(fsName: string, args: Map<
String,String>) to swap the filesystem at runtime.",
"NEW INSTRUCTION": "WHEN editing VFS module or API THEN keep single vfs.rs, hide impls, add replaceFS"
}

[2025-12-10 13:25] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "API scope",
"EXPECTATION": "Expose the needed filesystem methods directly on the Vfs trait instead of a separate API layer.",
"NEW INSTRUCTION": "WHEN proposing new filesystem operations THEN add them to the Vfs trait, not external helpers"
}

[2025-12-10 13:27] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "API scope",
"EXPECTATION": "Filesystem operations should be exposed directly on the Vfs trait, not via a separate API layer.",
"NEW INSTRUCTION": "WHEN proposing or adding filesystem methods THEN put them on the Vfs trait"
}

[2025-12-10 13:29] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "API scope",
"EXPECTATION": "Filesystem operations should be defined directly on the Vfs trait, not via a separate API layer.",
"NEW INSTRUCTION": "WHEN proposing or adding filesystem methods THEN put them on the Vfs trait"
}

[2025-12-10 13:31] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "global VFS singleton",
"EXPECTATION": "Use a single shared current_vfs for the whole project instead of multiple VFS instances.",
"NEW INSTRUCTION": "WHEN exposing VFS access THEN provide one global current_vfs behind RwLock"
}

[2025-12-10 13:32] - Updated by Junie
{
"TYPE": "correction",
"CATEGORY": "global VFS singleton",
"EXPECTATION": "Use one shared current_vfs for the whole project instead of multiple VFS instances.",
"NEW INSTRUCTION": "WHEN designing VFS lifecycle THEN expose a global current_vfs behind RwLock"
}

[2025-12-10 13:39] - Updated by Junie
{
"TYPE": "preference",
"CATEGORY": "verification approach",
"EXPECTATION": "Provide a mock/test filesystem, set it as active, and verify all file operations are routed through
it.",
"NEW INSTRUCTION": "WHEN validating VFS integration THEN use a test Vfs and assert all IO passes through it"
}

