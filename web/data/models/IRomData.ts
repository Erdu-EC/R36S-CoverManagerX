import type {OsString} from "~/data/types/OsString";
import type {IGameListEntry} from "~/data/models/IGameListEntry";
import {path} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/core";

export interface IRomData {
    name: OsString;
    extension: OsString;
    path: string;
    length: number;
}

export class RomData {
    private readonly name: OsString;
    private readonly extension: OsString;
    public path: string;
    public length: number;

    private gameListEntry?: IGameListEntry;

    constructor(data: IRomData) {
        this.name = data.name;
        this.extension = data.extension;
        this.path = data.path;
        this.length = data.length;
    }

    get nameString() {
        return this.gameListEntry ? this.gameListEntry.name : osStringToString(this.name);
    };

    get extensionString(): string {
        return osStringToString(this.extension);
    }

    get fullName(): string {
        return `${this.nameString}.${this.extensionString}`;
    }

    public async loadGameListEntry(gameListEntry: IGameListEntry, basePath: string) {
        if (gameListEntry.image)
            gameListEntry.image = await path.join(basePath, gameListEntry.image);
        this.gameListEntry = gameListEntry;
    }

    get imageUrl() {
        if (this.gameListEntry?.image) {
            return convertFileSrc(this.gameListEntry.image);
        }

        return null;
    }
}