import type {OsString} from "~/data/types/OsString";

export const osStringToString = (osString: OsString): string => {
    let decoder = new TextDecoder("utf-16");
    return decoder.decode(Uint16Array.from(osString));
}