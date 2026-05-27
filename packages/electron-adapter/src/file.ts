import { invoke } from "./invoke";
import type { IFileService } from "@agentsmesh/service-interface";

export class ElectronFileService implements IFileService {
  async upload_file(fileData: Uint8Array, filename: string, contentType: string): Promise<string> {
    return invoke<string>("fileUploadFile", Array.from(fileData), filename, contentType);
  }
}
