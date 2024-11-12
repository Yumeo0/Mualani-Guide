import PacketSnifferControl from './components/PacketSnifferControl';
import DeviceSelector from './components/DeviceSelector';
import { Event, listen, UnlistenFn } from '@tauri-apps/api/event';
import { StoreNotify } from './types';
import useStore from './hooks/useStore';
import { useEffect } from 'react';
import useResourcesStore from './hooks/useResourcesStore';

function App() {
    const { addItems } = useStore();
    const loadResources = useResourcesStore((state) => state.loadResources);

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
        // 'store_notify',
        'store_update',
        'team_swap_notify',
        // 'world_notify',
    ];

    useEffect(() => {
        console.log("Loading resources");
        loadResources().then(() => console.log("Finished loading resources")).then(async () => {
            /* const path = await tauriPath.resolveResource("resources/gi/data/TestData.json");
            const data = await readTextFile(path);
            let json = JSON.parse(data) as StoreNotify;
            addItems(json.items); */
        });
        let unlistenFunctions: UnlistenFn[] = [];

        async function run() {
            unlistenFunctions.forEach((unlisten) => unlisten());
            unlistenFunctions = []; // Reset the list

            for (const event of events) {
                console.debug('Listening for', event);
                const unlisten = await listen(event, (event: Event<string>) => {
                    console.log('Received', event.event, ':', JSON.parse(event.payload));
                });
                unlistenFunctions.push(unlisten);
            }

            const storeNotifyUnlisten = await listen('store_notify', (event: Event<string>) => {
                const data = JSON.parse(event.payload) as StoreNotify;
                console.log('store_notify', data);
                addItems(data.items);
            });
            unlistenFunctions.push(storeNotifyUnlisten);
        }

        run();

        return () => {
            console.log(unlistenFunctions)
            unlistenFunctions.forEach((unlisten) => unlisten());
        };
    }, [loadResources]);

    return (
        <main className="container">
            <h1>Sniffer</h1>
            <DeviceSelector />
            <PacketSnifferControl />
        </main>
    );
}

export default App;
