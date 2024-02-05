// FileSharing.sol
pragma solidity ^0.8.0;

contract FileSharing {
    struct File {
        string name;
        string content;
        address owner;
    }

    mapping(uint256 => File) public files;
    uint256 public fileCount;

    event FileUploaded(uint256 fileId, string name, address owner);

    function uploadFile(string memory _name, string memory _content) public {
        uint256 fileId = fileCount++;
        files[fileId] = File(_name, _content, msg.sender);
        emit FileUploaded(fileId, _name, msg.sender);
    }

    function downloadFile(uint256 fileId) public view returns (string memory, string memory, address) {
        require(fileId < fileCount, "File not found");
        File storage file = files[fileId];
        return (file.name, file.content, file.owner);
    }
}
