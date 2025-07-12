import json
import sqlite3
import sys

class Media:
    def __init__(self, fromDb):
        self.product_id = fromDb[0]
        self.content_ratings = json.loads(fromDb[1])
        self.descriptions = json.loads(fromDb[2])
        self.credits = json.loads(fromDb[3])
        self.child_products = json.loads(fromDb[4])
        self.last_updated = fromDb[5]
        self.last_updated_content = fromDb[6]
        self.media_type = fromDb[7]
        self.nostr_event_id = fromDb[8]
        self.images = json.loads(fromDb[9])
        self.videos = json.loads(fromDb[10])
        self.donation_address = fromDb[11]
        self.parent_product_id = fromDb[12]
        self.publisher_did = fromDb[13]
        self.release_status = fromDb[14]
        self.support_contact = fromDb[15]
        self.tags = json.loads(fromDb[16])
        self.titles = json.loads(fromDb[17])
        self.files = json.loads(fromDb[18])

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class Identity:
    def __init__(self, fromDb):
        self.did = fromDb[0]
        self.active_proof = fromDb[1]
        self.display_name = fromDb[2]
        self.avatar = fromDb[3]
        self.bio = fromDb[4]
        self.location = fromDb[5]
        self.languages = json.loads(fromDb[6])
        self.links = json.loads(fromDb[7])
        self.proofs = json.loads(fromDb[8])

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class Marketplace:
    def __init__(self, fromDb):
        self.id = fromDb[0]
        self.display_name = fromDb[1]
        self.url = fromDb[2]

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class DataPath:
    def __init__(self, fromDb):
        self.id = fromDb[0]
        self.display_name = fromDb[1]
        self.path = fromDb[2]

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class NostrKey:
    def __init__(self, fromDb):
        self.public_key = fromDb[0]
        self.private_key = fromDb[1]
        self.proof = fromDb[2]
        self.secured = fromDb[3]

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class NostrRelay:
    def __init__(self, fromDb):
        self.id = fromDb[0]
        self.display_name = fromDb[1]
        self.url = fromDb[2]

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)
    

class ActiveConfig:
    def __init__(self, fromDb):
        self.id = fromDb[0]
        self.did = fromDb[1]
        self.active_proof = fromDb[2]
        self.marketplace_display_name = fromDb[3]
        self.marketplace_url = fromDb[4]
        self.torrent_client_port = fromDb[5]
        self.languages = fromDb[6]
        self.install_path = fromDb[7]
        self.install_path_display_name = fromDb[8]
        self.torrent_path = fromDb[9]
        self.torrent_path_display_name = fromDb[10]
        self.minting_data_path = fromDb[11]

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)
    
    def _Repr__(self):
        return json.dumps(self.__dict__, indent=4)


class SlimeDB:
    def __init__(self, db_path='../resources/slime.db'):
        self.db_path = db_path
        self.conn = sqlite3.connect(self.db_path)
        self.cursor = self.conn.cursor()
        self.create_tables()

    def create_tables(self):
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS media (
                productId TEXT PRIMARY KEY,
                contentRatings JSON,
                descriptions JSON,
                credits JSON,
                childProducts JSON,
                lastUpdated INTEGER,
                lastUpdatedContent INTEGER,
                mediaType TEXT,
                nostrEventId TEXT,
                images JSON,
                videos JSON,
                donationAddress TEXT,
                parentProductId TEXT,
                publisherDid TEXT,
                releaseStatus TEXT,
                supportContact TEXT,
                tags JSON,
                titles JSON,
                files JSON
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS identities (
                did TEXT PRIMARY KEY,
                activeProof JSON,
                displayName TEXT,
                avatar TEXT,
                bio TEXT,
                location TEXT,
                languages JSON,
                links JSON,
                proofs JSON
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS marketplaces (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                url TEXT
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS installPaths (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                path TEXT
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS torrentPaths (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                path TEXT
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS nostrRelays (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                url TEXT
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS nostrKeys (
                publicKey TEXT PRIMARY KEY AUTOINCREMENT,
                privateKey TEXT,
                proof TEXT,
                secured INTEGER
            )
        ''')
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS activeConfig (
                id INTEGER PRIMARY KEY,
                did TEXT,
                activeProof JSON,
                marketplaceDisplayName TEXT,
                marketplaceUrl TEXT,
                torrentClientPort INTEGER,
                languages JSON,
                installPath TEXT,
                installPathDisplayName TEXT,
                torrentPath TEXT,
                torrentPathDisplayName TEXT,
                mintingDataPath TEXT
            )
        ''')
        self.cursor.execute('''
            INSERT OR IGNORE INTO activeConfig (id, installPath, torrentPath, mintingDataPath, torrentClientPort, languages, installPathDisplayName, torrentPathDisplayName) VALUES (1, './installs', './torrents', './minting', 5235, '[\"english\"]', 'Default', 'Default')
        ''')
        self.conn.commit()

    def add_media(self, media: Media):
        self.cursor.execute('''
            INSERT OR REPLACE INTO media (
                productId, contentRatings, descriptions, credits,
                childProducts, lastUpdated, lastUpdatedContent,
                mediaType, nostrEventId, images, videos,
                donationAddress, parentProductId, publisherDid,
                releaseStatus, supportContact, tags, titles, files
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (
            media.product_id, json.dumps(media.content_ratings),
            json.dumps(media.descriptions), json.dumps(media.credits),
            json.dumps(media.child_products), media.last_updated,
            media.last_updated_content, media.media_type,
            media.nostr_event_id, json.dumps(media.images),
            json.dumps(media.videos), media.donation_address,
            media.parent_product_id, media.publisher_did,
            media.release_status, media.support_contact,
            json.dumps(media.tags), json.dumps(media.titles),
            json.dumps(media.files)
        ))

        self.conn.commit()

    def get_media(self, product_id):
        result = self.cursor.execute('''
            SELECT * FROM media WHERE productId = ?
        ''', (product_id,)).fetchone()
        if result:
            return Media(result)
        return None
    
    def get_all_media(self, page_size, page_number, sort_by="titles"):
        offset = (page_number - 1) * page_size
        query = '''
            SELECT * FROM media
            '''
        if sort_by:
            query += f'ORDER BY {sort_by} '
        query += 'LIMIT ? OFFSET ?'
        results = self.cursor.execute(query, (page_size, offset)).fetchall()
        return [Media(row) for row in results]

    def add_identity(self, did, active_proof, display_name=None, avatar=None, bio=None, location=None, languages=None, links=None, proofs=None):
        self.cursor.execute('''
            INSERT OR REPLACE INTO identities (did, activeProof, displayName, avatar, bio, location, languages, links, proofs)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ''', (did, active_proof, display_name, avatar, bio, location, json.dumps(languages), json.dumps(links), json.dumps(proofs)))
        self.conn.commit()

    def get_identities(self):
        results = self.cursor.execute('''
            SELECT * FROM identities
        ''').fetchall()
        return [Identity(row) for row in results]
    
    def delete_identity(self, did):
        self.cursor.execute('''
            DELETE FROM identities WHERE did = ?
        ''', (did,))
        self.conn.commit()

    def add_installPath(self, path: DataPath):
        if path.id == 0:
            self.cursor.execute('''
                INSERT OR REPLACE INTO installPaths (displayName, path)
                VALUES (?, ?, ?)
            ''', (path.display_name, path.path))
        else:
            self.cursor.execute('''
                INSERT OR REPLACE INTO installPaths (id, displayName, path)
                VALUES (?, ?, ?)
            ''', (path.id, path.display_name, path.path))
        self.conn.commit()
    
    def get_installPaths(self):
        results = self.cursor.execute('''
            SELECT * FROM installPaths
        ''').fetchall()
        return [DataPath(row) for row in results]
    
    def delete_installPath(self, path_id):
        self.cursor.execute('''
            DELETE FROM installPaths WHERE id = ?
        ''', (path_id,))
        self.conn.commit()
    
    def add_torrentPath(self, path: DataPath):
        if path.id == 0:
            self.cursor.execute('''
                INSERT OR REPLACE INTO torrentPaths (displayName, path)
                VALUES (?, ?, ?)
            ''', (path.display_name, path.path))
        else:
            self.cursor.execute('''
                INSERT OR REPLACE INTO torrentPaths (id, displayName, path)
                VALUES (?, ?, ?)
            ''', (path.id, path.display_name, path.path))
        self.conn.commit()

    def get_torrentPaths(self):
        results = self.cursor.execute('''
            SELECT * FROM torrentPaths
        ''').fetchall()
        return [DataPath(row) for row in results]
    
    def add_marketplace(self, marketplace: Marketplace):
        print("adding marketplace", marketplace)
        if marketplace.id <= 0:
            self.cursor.execute('''
                INSERT OR REPLACE INTO marketplaces (displayName, url)
                VALUES (?, ?, ?)
            ''', (marketplace.display_name, marketplace.url))
        else:
            self.cursor.execute('''
                INSERT OR REPLACE INTO marketplaces (id, displayName, url)
                VALUES (?, ?, ?)
            ''', (marketplace.id, marketplace.display_name, marketplace.url))
        self.conn.commit()

    def get_marketplaces(self):
        results = self.cursor.execute('''
            SELECT * FROM marketplaces
        ''').fetchall()
        return [Marketplace(row) for row in results]
    
    def delete_marketplace(self, marketplace_id):
        self.cursor.execute('''
            DELETE FROM marketplaces WHERE id = ?
        ''', (marketplace_id,))
        self.conn.commit()
    
    def addNostrRelay(self, relay: NostrRelay):
        if relay.id == 0:
            self.cursor.execute('''
                INSERT OR REPLACE INTO nostrRelays (displayName, url)
                VALUES (?, ?, ?)
            ''', (relay.display_name, relay.url))
        else:
            self.cursor.execute('''
                INSERT OR REPLACE INTO nostrRelays (id, displayName, url)
                VALUES (?, ?, ?)
            ''', (relay.id, relay.display_name, relay.url))
        self.conn.commit()

    def getNostrRelays(self):
        results = self.cursor.execute('''
            SELECT * FROM nostrRelays
        ''').fetchall()
        return [NostrRelay(row) for row in results]
    
    def deleteNostrRelay(self, relay_id):
        self.cursor.execute('''
            DELETE FROM nostrRelays WHERE id = ?
        ''', (relay_id,))
        self.conn.commit()
    
    def getNostrKey(self, publicKey):
        result = self.cursor.execute('''
            SELECT * FROM nostrKeys WHERE publicKey = ?
        ''', (publicKey,)).fetchone()
        if result:
            return NostrKey(result)
        return None

    def set_active_identity(self, did):
        identity = self.cursor.execute('''
            SELECT * FROM identities WHERE did = ?
        ''', (did,)).fetchone()
        self.cursor.execute('''
            UPDATE activeConfig
            SET did = ?, currentNostrPublicKey = ?, proof = ?
            WHERE id = 1
        ''', (identity[0], identity[1], identity[2]))
        self.conn.commit()

    def get_active_config(self):
        result = self.cursor.execute('''
            SELECT * FROM activeConfig WHERE id = 1
        ''').fetchone()
        if result:
            return ActiveConfig(result)
        return None

    def close(self):
        self.conn.close()