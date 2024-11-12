import type {OsString} from "~/data/types/OsString";

export interface IGameListEntry {
    path: OsString;
    name: OsString;
    image: OsString;
}

export class GameListEntry implements IGameListEntry {
    private _image: OsString;
    private readonly _name: OsString;
    private readonly _path: OsString;

    constructor(gameListEntry: IGameListEntry) {
        this._image = gameListEntry.image;
        this._name = gameListEntry.name;
        this._path = gameListEntry.path;
    }

    get path(): string {
        return osStringToString(this._path);
    }

    get name(): string {
        return osStringToString(this._name);
    }

    get image(): string {
        return osStringToString(this._image);
    }

    set image(image: string) {
        this._image = image;
    }
}