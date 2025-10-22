import { TrayIcon } from '@tauri-apps/api/tray';
import { Menu } from '@tauri-apps/api/menu';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { exit } from '@tauri-apps/plugin-process';
import { Window } from '@tauri-apps/api/window';

export const InitTray = async () => {

    const onTrayMenuClick = async (itemId) => {
        switch (itemId) {
            case 'quit':
                await exit(0);
                break;
            case 'show':
                await Window.getAll().then(win => win[0].show());
                await Window.getAll().then(win => win[0].setFocus());
                break;
        }
    }
    const menu = await Menu.new({
        items: [
            {
                id: 'show',
                text: 'Show Window',
                action: onTrayMenuClick
            },
            {
                id: 'quit',
                text: 'Quit',
                action: onTrayMenuClick
            },
        ],
    });

    const options = {
        icon: await defaultWindowIcon(),
        menu,
        menuOnLeftClick: true,
    };

    const tray = await TrayIcon.new(options);
}