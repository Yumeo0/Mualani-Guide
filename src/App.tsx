import PacketSnifferControl from './components/PacketSnifferControl';
import DeviceSelector from './components/DeviceSelector';
import { Event, listen } from '@tauri-apps/api/event';

function App() {

  listen('player_token_rsp', (event: Event<string>) => {
    console.log('Received PlayerTokenRsp:', event.payload);
  });

  listen('player_info', (event: Event<string>) => {
    console.log('Received PlayerInfo:', event.payload);
  });

  return (
    <main className="container">
      <h1>Sniffer</h1>
      <DeviceSelector />
      <PacketSnifferControl />
    </main>
  );
}

export default App;
