import { CustomLocalStorageBase } from '@store/locol';
import { ActiveCategory } from './global.slice';

// eslint-disable-next-line no-shadow
enum GlobalLocal {
    ACTIVE_CATEGORY_ID = 'category_id',
    ACTIVE_CATEGORY_NAME = 'category_name',
}

export default class GlobalLocalStorage extends CustomLocalStorageBase<GlobalLocal> {
    // eslint-disable-next-line no-use-before-define
    private static instance?: GlobalLocalStorage;

    // eslint-disable-next-line no-useless-constructor
    private constructor() {
        super();
    }

    public static getInstance() {
        if (!this.instance) {
            this.instance = new GlobalLocalStorage();
        }

        return this.instance;
    }

    public setActiveCategory(categoryId: string, categoryName: string) {
        this.set(GlobalLocal.ACTIVE_CATEGORY_ID, categoryId);
        this.set(GlobalLocal.ACTIVE_CATEGORY_NAME, categoryName);
    }

    public getActiveCategory(): ActiveCategory {
        return {
            id:   this.get(GlobalLocal.ACTIVE_CATEGORY_ID) || '',
            name: this.get(GlobalLocal.ACTIVE_CATEGORY_NAME) || '',
        };
    }

    public clearActiveCategory() {
        this.clearItems([GlobalLocal.ACTIVE_CATEGORY_ID, GlobalLocal.ACTIVE_CATEGORY_NAME]);
    }
}
