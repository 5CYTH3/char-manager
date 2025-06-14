import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import type { CharData } from "../schemas/char";

export const load: PageLoad = async ({ params }) => {
    let infos: CharData[] = await invoke("get_chars_infos");
    return {
        infos,
    };
};
