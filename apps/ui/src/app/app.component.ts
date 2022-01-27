import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';

@Component({
  selector: 'guitar-audio-processor-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  title = 'ui';

  async testRust() {
    console.log('invoking rust command...');
    const rustResp = await invoke('test_command');

    console.log(`rustResp: "${rustResp}"`);
  }
}
