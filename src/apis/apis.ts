import {rcall} from "./tauri";

export const echo = async (data: any) => {
   let respone = await rcall("echo",{req: JSON.stringify(data)})
   return respone
  };
