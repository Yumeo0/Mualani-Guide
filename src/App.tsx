import PacketSnifferControl from './components/PacketSnifferControl';
import DeviceSelector from './components/DeviceSelector';
import { Event, listen } from '@tauri-apps/api/event';

function App() {

  const events = [
    'achievement_notify',
    //'achievement_update_notify',
    'api_key_notify',
    'avatar_fight_prop_update',
    'avatar_notify',
    'avatar_property_update',
    'friend_init',
    'player_info',
    'player_token_rsp',
    'player_update',
    'quest_notify',
    'scene_entity_die_update',
    'scene_update',
    'avatar_skill_update',
    'store_notify',
    'store_update',
    'team_swap_notify',
    'world_notify',
  ];

  events.forEach((event) => {
    console.debug('Listening for', event);
    listen(event, (event: Event<string>) => {
      console.log('Received', event.event, ':', JSON.parse(event.payload));
    });
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
