# HorizonFS NameNode

## Client Operation Supported
1.Create
Initiates the creation of a new file. The NameNode checks namespace availability and allocates initial blocks.
2.GetFileInfo.
Retrieves metadata (size, permissions, modification times, etc.) about a file or directory.
3.GetBlockLocations.
Returns the list of DataNodes where each block of a file is stored, which is essential for read operations.
4.AddBlock.
Allocates a new block when a client is writing data, updating the file’s block list.
5.Complete.
Finalizes the file write operation, ensuring all blocks are properly replicated before closing the file.

### Delete
Removes files or directories from the namespace.

### Rename.
Changes the path of an existing file or directory.

### SetReplication:
Updates the replication factor for a given file.

# Mkdirs:
Creates directories in the namespace.

GetContentSummary:
Provides aggregate statistics (like size, file count, etc.) for a directory tree.

2. DataNode Protocol Messages
   These messages are used by DataNodes to communicate their status and block information to the NameNode:

RegisterDatanode:
DataNodes use this message to register themselves with the NameNode when they start up.

SendHeartbeat:
Periodic heartbeats are sent by DataNodes to inform the NameNode that they are alive and functioning. These messages often carry simple metrics such as available storage.

BlockReport:
A full report of all blocks stored on a DataNode, sent at regular intervals or when triggered by an event. This helps the NameNode maintain an accurate mapping of block locations.

BlockReceivedAndDeleted:
Notifies the NameNode when blocks have been received or deleted, which is important for maintaining consistency during replication and cleanup.

Error Reporting:
Used by DataNodes to report errors or issues with specific blocks or the node itself.

Additional Considerations
Safe Mode Management:
The NameNode must support messages related to entering or exiting safe mode. In safe mode, the NameNode does not allow changes to the file system (e.g., block allocations or file deletions) until a certain percentage of blocks have been reported by DataNodes.

Administrative Commands:
For operational purposes, the NameNode might also need to handle administrative messages such as refreshing the list of DataNodes (e.g., after configuration changes) or updating other dynamic settings.

Version and Protocol Negotiation:
It’s important to manage version compatibility between the NameNode and its clients/DataNodes. Including a mechanism for version checking can help ensure that all parties speak the same “language.”