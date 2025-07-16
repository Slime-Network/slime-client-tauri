import platform
import sqlite3
from jsonrpclib.SimpleJSONRPCServer import (
    SimpleJSONRPCServer,
    SimpleJSONRPCRequestHandler,
)
from base64 import b64decode, b64encode

import os
import sys
import shutil

from slime_db import SlimeDB
from torrents import TorrentHandler

import pyzipper


class RequestHandler(SimpleJSONRPCRequestHandler):
    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def end_headers(self):
        self.send_header(
            "Access-Control-Allow-Headers",
            "Origin, X-Requested-With, Content-Type, Accept",
        )
        self.send_header("Access-Control-Allow-Origin", "localhost")
        self.send_header("Max-Http-Header-Size", "1000000000")
        SimpleJSONRPCRequestHandler.end_headers(self)

db = sqlite3.connect("./_up_/resources/slime.db", check_same_thread=False)
c = db.cursor()

class SlimeRPC:
    def __init__(self) -> None:
        self.config = SlimeDB().get_active_config()
        print("Python - Config loaded : ", self.config, file=sys.stderr)

        host = "localhost"
        port = self.config.torrent_client_port

        print(f"Python - Server starting at {host}:{port}", file=sys.stderr)

        self.server = SimpleJSONRPCServer(
            (host, port), requestHandler=RequestHandler, bind_and_activate=True
        )

        print(f"Python - Server started at {host}:{port}", file=sys.stderr)
        print("Python - Current directory:", os.getcwd(), file=sys.stderr)
        
        print(self.config)
        os.makedirs(os.path.dirname(self.config.install_path), exist_ok=True)
        os.makedirs(os.path.dirname(self.config.torrent_path), exist_ok=True)
        os.makedirs(os.path.dirname(self.config.minting_data_path), exist_ok=True)

        self.torrent_handler = TorrentHandler()
        self.server.register_function(self.ping, "ping")
        self.server.register_function(self.download_media, "downloadMedia")
        self.server.register_function(self.delete_media, "deleteMedia")
        self.server.register_function(self.install_media, "installMedia")
        self.server.register_function(self.uninstall_media, "uninstallMedia")
        self.server.register_function(self.get_install_status, "getInstallStatus")
        self.server.register_function(self.generate_torrent, "generateTorrent")
        self.server.register_function(self.get_torrent_status, "getTorrentStatus")
        self.server.register_function(self.get_operating_system, "getOperatingSystem")
        self.server.register_function(self.kill, "kill")

        print("Python - Server registered functions")

        print("using config", self.server)

    def serve(self):
        try:
            self.server.serve_forever()
        except Exception as e:
            print(e)

    def kill(self):
        print("kill")
        self.server.shutdown()

    def ping(self):
        print("ping")
        return {"message", "pong"}

    def download_media(self, media):
        try:
            print("Downloading: " + media["title"])
            filename = get_filename(media, get_operating_system())
            with open(
                self.config.torrent_path + filename + ".torrent",
                "wb",
            ) as f:
                f.write(b64decode(s=media["torrents"][get_operating_system()]))
                f.close()

            self.torrent_handler.add_torrent(
                torrentpath=self.config.torrent_path, filename=filename + ".torrent"
            )
            return {"status": "Downloading"}
        except Exception as e:
            print("Error in download_media" + str(e))
            return {"status": "error", "message": "Error in download_media: " + str(e)}

    def delete_media(self, media):
        try:
            print("Deleting: " + media["title"])
            filename = get_filename(media, get_operating_system())
            os.remove(self.config.torrent_path + filename + ".torrent")
            os.remove(self.config.torrent_path + filename + ".zip")
            os.remove(self.config.torrent_path + filename + ".zip")
            self.torrent_handler.remove_torrent(filename + ".torrent")
            return {"status": "Deleted"}
        except Exception as e:
            print("Error in delete_media" + str(e))
            return {"status": "error", "message": "Error in delete_media: " + str(e)}

    def install_media(self, media):
        try:
            print("Installing: " + media["title"])

            with pyzipper.AESZipFile(
                self.config.torrent_path
                + "/"
                + get_filename(media, get_operating_system())
                + ".zip",
                "r",
                compression=pyzipper.ZIP_DEFLATED,
                encryption=pyzipper.WZ_AES,
            ) as zip:
                zip.extractall(
                    self.config.install_path
                    + get_filename(media, get_operating_system()),
                    pwd=str.encode(media["password"]),
                )

            return {"status": "complete"}
        except Exception as e:
            print("Error in install_media" + str(e))
            return {"status": "error", "message": "Error in install_media: " + str(e)}

    def uninstall_media(self, media):
        try:
            print("Uninstalling: " + media["title"])
            filename = get_filename(media, get_operating_system())
            shutil.rmtree(self.config.install_path + filename)
            return {"status": "Uninstalled"}
        except Exception as e:
            print("Error in uninstall_media" + str(e))
            return {"status": "error", "message": "Error in uninstall_media: " + str(e)}

    def get_install_status(self, media):
        pass
        try:
            print("get_install_status")
            status = self.torrent_handler.get_status(
                get_filename(media, "windows") + ".zip"
            )
            print(
                f"{status.name}-> {status.state}: {status.progress}% - {status.download_rate}v | ^{status.upload_rate}"
            )

            return {
                "status": {
                    "isDownloaded": os.path.exists(
                        self.config.torrent_path
                        + get_filename(media, get_operating_system())
                        + ".zip"
                    ),
                    "isDownloading": (str(status.state) == "downloading"),
                    "isInstalled": os.path.exists(
                        self.config.install_path
                        + get_filename(media, get_operating_system())
                    ),
                    "isInstalling": False,
                    "hasPendingUpdate": False,
                    "isSeeding": (str(status.state) == "seeding"),
                    "progress": status.progress,
                    "downloadRate": status.download_rate,
                    "uploadRate": status.upload_rate,
                },
                "message": "Status retrieved",
            }
        except Exception as e:
            print("Error in get_install_status" + str(e))
            return {
                "status": "error",
                "message": "Error in get_install_status: " + str(e),
            }

    def generate_torrent(self, mediaFiles, source, destination):
        try:
            print("generate_torrent PYTHON", file=sys.stderr)
            print("python mediaFiles", mediaFiles, file=sys.stderr)
            print("python path", source, file=sys.stderr)

            result = {}
            size = 0
            contents = os.walk(source)
            with pyzipper.AESZipFile(
                destination,
                "w",
                compression=pyzipper.ZIP_DEFLATED,
                encryption=pyzipper.WZ_AES,
            ) as zf:
                zf.setpassword(bytes(mediaFiles["password"], "utf-8"))
                for root, folders, files in contents:
                    for folder_name in folders:
                        absolute_path = os.path.join(root, folder_name)
                        relative_path = absolute_path.replace(
                            destination + "\\", ""
                        )
                        print("Adding '%s' to archive." % absolute_path)
                        zf.write(absolute_path, relative_path)
                    for file_name in files:
                        absolute_path = os.path.join(root, file_name)
                        relative_path = absolute_path.replace(
                            destination + "\\", ""
                        )
                        print("Adding '%s' to archive." % absolute_path)
                        zf.write(absolute_path, relative_path)
                    tor = self.torrent_handler.make_torrent(
                        destination, destination + ".torrent"
                    )
                
                    result = b64encode(tor).decode("utf-8")
            
            size = os.path.getsize(destination)
            print("generate_torrent" + result, file=sys.stderr)
            return {
                "torrent": result,
                "fileName": destination,
                "size": size,
            }
        except Exception as e:
            print("Error in generate_torrent" + str(e))
            return "Error in generate_torrent: " + str(e)

    def get_torrent_status(self, media):
        pass
        try:
            result = self.torrent_handler.get_status(
                get_filename(media, get_operating_system()) + ".zip"
            )
            print(result)
            return {"status": result}
        except Exception as e:
            print("Error in get_torrent_status" + str(e))
            return {
                "status": "error",
                "message": "Error in get_torrent_status: " + str(e),
            }

    def get_operating_system(self):
        try:
            print("get_operating_system")
            return {"os": get_operating_system()}
        except Exception as e:
            print("Error in get_operating_system" + str(e))
            return {"message": "Error in get_operating_system: " + str(e)}


def get_filename(media, operatingSystem):
    return media["productId"].replace(" ", "-") + "-" + operatingSystem


def get_operating_system():
    if platform.system() == "Windows":
        return "windows"
    elif platform.system() == "Darwin":
        return "mac"
    elif platform.system() == "Linux":
        return "linux"
    else:
        return "unknown"


if __name__ == "__main__":
    slime = SlimeRPC()
    slime.serve()