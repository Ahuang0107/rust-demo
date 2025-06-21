#[macro_export]
macro_rules! i18n {
    // 基本形式：i18n!(en=>"hello", ch=>"你好")
    ($($lang:ident => $text:expr),+ $(,)?) => {
        {
            match $crate::i18n3::get_lang() {
                $(
                    $crate::Language::$lang => $text,
                )+
                _ => "", // 如果没有匹配的语言，返回空字符串，在开发阶段保留，方便只测试少数本地化文本，但是最终打包时需要去掉，确保所有语言都有配置
            }
        }
    };
}

pub enum Text {
    Start,
    Exit,
    Save,
    Load,
    Play,
    Pause,
    Settings,
    Level,
    Health,
    Energy,
    Attack,
    Defense,
    Magic,
    Skill,
    Quest,
    Item,
    Gold,
    Shop,
    Map,
    Enemy,
    Boss,
    Victory,
    Defeat,
    Character,
    Inventory,
    Weapon,
    Armor,
    Potion,
    Key,
    Door,
    Treasure,
    Dungeon,
    Village,
    Castle,
    Forest,
    Desert,
    Ocean,
    Mountain,
    Sky,
    Fire,
    Water,
    Wind,
    Earth,
    Light,
    Dark,
    Time,
    Space,
    Power,
    Speed,
    Luck,
    Arrow,
    Banner,
    Cloak,
    Dagger,
    Elixir,
    Forge,
    Goblin,
    Herald,
    Illusion,
    Jester,
    Keystone,
    Labyrinth,
    Mystic,
    Nexus,
    Oath,
    Pendant,
    Quiver,
    Rune,
    Sigil,
    Tome,
    Urn,
    Vial,
    Warlock,
    Yew,
    Zealot,
    Altar,
    Brew,
    Cairn,
    Druid,
    Effigy,
    Frost,
    Glyph,
    Hovel,
    Ire,
    Jolt,
    Knoll,
    Lich,
    Maw,
    Nectar,
    Omen,
    Plague,
    Quartz,
    Revenant,
    Shroud,
    Tundra,
    Undead,
    Vortex,
    Wight,
    Xeric,
    Yonder,
    Eldritch,
    Phylactery,
    Zephyr,
    PressStar,
    GameOver,
    Continue,
    NewGame,
    HighScore,
    SoundOnO,
    FullScree,
    VolumeUp,
    Checkpoint,
    UnlockAch,
    Loading,
    PleaseWai,
    Connection,
    Retry,
    ConfirmDe,
    NotEnough,
    LevelUp,
    CriticalH,
    OutofMan,
    PartyFull,
    Welcometo,
    Youfound,
    Theenemy,
    Yourhealt,
    Wouldyou,
    Thisdoor,
    Youneeda,
    Amysterio,
    Thebattle,
    Youdefeat,
    Criticalh,
    Enemyiss,
    Yourshiel,
    Ahiddenp,
    Yousense,
    Newskill,
    Strengt,
    Yourreput,
    Autosave,
    Controller,
    Downloadin,
    Theforest,
    Illtrade,
    Bewarethe,
    Questupda,
    Optionalo,
    Timeremai,
    Iusedto,
    Thisisnt,
    Theancien,
    Tounlock,
    Thekingdo,
}

impl Text {
    pub fn text(&self) -> &'static str {
        match self {
            Text::Start => {
                i18n!(en=>"Start", sc=>"开始", tc=>"開始", jp=>"スタート", ko=>"시작", es=>"Inicio", fr=>"Démarrer", ru=>"Начать")
            }
            Text::Exit => {
                i18n!(en=>"Exit", sc=>"退出", tc=>"退出", jp=>"終了", ko=>"종료", es=>"Salir", fr=>"Quitter", ru=>"Выход")
            }
            Text::Save => {
                i18n!(en=>"Save", sc=>"保存", tc=>"保存", jp=>"セーブ", ko=>"저장", es=>"Guardar", fr=>"Sauvegarder", ru=>"Сохранить")
            }
            Text::Load => {
                i18n!(en=>"Load", sc=>"加载", tc=>"載入", jp=>"ロード", ko=>"불러오기", es=>"Cargar", fr=>"Charger", ru=>"Загрузить")
            }
            Text::Play => {
                i18n!(en=>"Play", sc=>"游玩", tc=>"遊玩", jp=>"プレイ", ko=>"플레이", es=>"Jugar", fr=>"Jouer", ru=>"Играть")
            }
            Text::Pause => {
                i18n!(en=>"Pause", sc=>"暂停", tc=>"暫停", jp=>"一時停止", ko=>"일시 정지", es=>"Pausa", fr=>"Pause", ru=>"Пауза")
            }
            Text::Settings => {
                i18n!(en=>"Settings", sc=>"设置", tc=>"設定", jp=>"設定", ko=>"설정", es=>"Configuración", fr=>"Paramètres", ru=>"Настройки")
            }
            Text::Level => {
                i18n!(en=>"Level", sc=>"等级", tc=>"等級", jp=>"レベル", ko=>"레벨", es=>"Nivel", fr=>"Niveau", ru=>"Уровень")
            }
            Text::Health => {
                i18n!(en=>"Health", sc=>"生命值", tc=>"生命值", jp=>"体力", ko=>"체력", es=>"Salud", fr=>"Santé", ru=>"Здоровье")
            }
            Text::Energy => {
                i18n!(en=>"Energy", sc=>"能量", tc=>"能量", jp=>"エネルギー", ko=>"에너지", es=>"Energía", fr=>"Énergie", ru=>"Энергия")
            }
            Text::Attack => {
                i18n!(en=>"Attack", sc=>"攻击", tc=>"攻擊", jp=>"攻撃", ko=>"공격", es=>"Ataque", fr=>"Attaque", ru=>"Атака")
            }
            Text::Defense => {
                i18n!(en=>"Defense", sc=>"防御", tc=>"防禦", jp=>"防御", ko=>"방어", es=>"Defensa", fr=>"Défense", ru=>"Защита")
            }
            Text::Magic => {
                i18n!(en=>"Magic", sc=>"魔法", tc=>"魔法", jp=>"魔法", ko=>"마법", es=>"Magia", fr=>"Magie", ru=>"Магия")
            }
            Text::Skill => {
                i18n!(en=>"Skill", sc=>"技能", tc=>"技能", jp=>"スキル", ko=>"스킬", es=>"Habilidad", fr=>"Compétence", ru=>"Навык")
            }
            Text::Quest => {
                i18n!(en=>"Quest", sc=>"任务", tc=>"任務", jp=>"クエスト", ko=>"퀘스트", es=>"Misión", fr=>"Quête", ru=>"Задание")
            }
            Text::Item => {
                i18n!(en=>"Item", sc=>"物品", tc=>"物品", jp=>"アイテム", ko=>"아이템", es=>"Objeto", fr=>"Objet", ru=>"Предмет")
            }
            Text::Gold => {
                i18n!(en=>"Gold", sc=>"金币", tc=>"金幣", jp=>"ゴールド", ko=>"골드", es=>"Oro", fr=>"Or", ru=>"Золото")
            }
            Text::Shop => {
                i18n!(en=>"Shop", sc=>"商店", tc=>"商店", jp=>"ショップ", ko=>"상점", es=>"Tienda", fr=>"Boutique", ru=>"Магазин")
            }
            Text::Map => {
                i18n!(en=>"Map", sc=>"地图", tc=>"地圖", jp=>"マップ", ko=>"지도", es=>"Mapa", fr=>"Carte", ru=>"Карта")
            }
            Text::Enemy => {
                i18n!(en=>"Enemy", sc=>"敌人", tc=>"敵人", jp=>"敵", ko=>"적", es=>"Enemigo", fr=>"Ennemi", ru=>"Враг")
            }
            Text::Boss => {
                i18n!(en=>"Boss", sc=>"首领", tc=>"首領", jp=>"ボス", ko=>"보스", es=>"Jefe", fr=>"Boss", ru=>"Босс")
            }
            Text::Victory => {
                i18n!(en=>"Victory", sc=>"胜利", tc=>"勝利", jp=>"勝利", ko=>"승리", es=>"Victoria", fr=>"Victoire", ru=>"Победа")
            }
            Text::Defeat => {
                i18n!(en=>"Defeat", sc=>"失败", tc=>"失敗", jp=>"敗北", ko=>"패배", es=>"Derrota", fr=>"Défaite", ru=>"Поражение")
            }
            Text::Character => {
                i18n!(en=>"Character", sc=>"角色", tc=>"角色", jp=>"キャラクター", ko=>"캐릭터", es=>"Personaje", fr=>"Personnage", ru=>"Персонаж")
            }
            Text::Inventory => {
                i18n!(en=>"Inventory", sc=>"背包", tc=>"背包", jp=>"インベントリ", ko=>"인벤토리", es=>"Inventario", fr=>"Inventaire", ru=>"Инвентарь")
            }
            Text::Weapon => {
                i18n!(en=>"Weapon", sc=>"武器", tc=>"武器", jp=>"武器", ko=>"무기", es=>"Arma", fr=>"Arme", ru=>"Оружие")
            }
            Text::Armor => {
                i18n!(en=>"Armor", sc=>"护甲", tc=>"護甲", jp=>"防具", ko=>"방어구", es=>"Armadura", fr=>"Armure", ru=>"Броня")
            }
            Text::Potion => {
                i18n!(en=>"Potion", sc=>"药水", tc=>"藥水", jp=>"ポーション", ko=>"포션", es=>"Poción", fr=>"Potion", ru=>"Зелье")
            }
            Text::Key => {
                i18n!(en=>"Key", sc=>"钥匙", tc=>"鑰匙", jp=>"鍵", ko=>"열쇠", es=>"Llave", fr=>"Clé", ru=>"Ключ")
            }
            Text::Door => {
                i18n!(en=>"Door", sc=>"门", tc=>"門", jp=>"ドア", ko=>"문", es=>"Puerta", fr=>"Porte", ru=>"Дверь")
            }
            Text::Treasure => {
                i18n!(en=>"Treasure", sc=>"宝藏", tc=>"寶藏", jp=>"宝物", ko=>"보물", es=>"Tesoro", fr=>"Trésor", ru=>"Сокровище")
            }
            Text::Dungeon => {
                i18n!(en=>"Dungeon", sc=>"地牢", tc=>"地牢", jp=>"ダンジョン", ko=>"던전", es=>"Mazmorra", fr=>"Donjon", ru=>"Подземелье")
            }
            Text::Village => {
                i18n!(en=>"Village", sc=>"村庄", tc=>"村莊", jp=>"村", ko=>"마을", es=>"Aldea", fr=>"Village", ru=>"Деревня")
            }
            Text::Castle => {
                i18n!(en=>"Castle", sc=>"城堡", tc=>"城堡", jp=>"城", ko=>"성", es=>"Castillo", fr=>"Château", ru=>"Замок")
            }
            Text::Forest => {
                i18n!(en=>"Forest", sc=>"森林", tc=>"森林", jp=>"森", ko=>"숲", es=>"Bosque", fr=>"Forêt", ru=>"Лес")
            }
            Text::Desert => {
                i18n!(en=>"Desert", sc=>"沙漠", tc=>"沙漠", jp=>"砂漠", ko=>"사막", es=>"Desierto", fr=>"Désert", ru=>"Пустыня")
            }
            Text::Ocean => {
                i18n!(en=>"Ocean", sc=>"海洋", tc=>"海洋", jp=>"海", ko=>"바다", es=>"Océano", fr=>"Océan", ru=>"Океан")
            }
            Text::Mountain => {
                i18n!(en=>"Mountain", sc=>"山脉", tc=>"山脈", jp=>"山", ko=>"산", es=>"Montaña", fr=>"Montagne", ru=>"Гора")
            }
            Text::Sky => {
                i18n!(en=>"Sky", sc=>"天空", tc=>"天空", jp=>"空", ko=>"하늘", es=>"Cielo", fr=>"Ciel", ru=>"Небо")
            }
            Text::Fire => {
                i18n!(en=>"Fire", sc=>"火焰", tc=>"火焰", jp=>"火", ko=>"불", es=>"Fuego", fr=>"Feu", ru=>"Огонь")
            }
            Text::Water => {
                i18n!(en=>"Water", sc=>"水", tc=>"水", jp=>"水", ko=>"물", es=>"Agua", fr=>"Eau", ru=>"Вода")
            }
            Text::Wind => {
                i18n!(en=>"Wind", sc=>"风", tc=>"風", jp=>"風", ko=>"바람", es=>"Viento", fr=>"Vent", ru=>"Ветер")
            }
            Text::Earth => {
                i18n!(en=>"Earth", sc=>"大地", tc=>"大地", jp=>"土", ko=>"흙", es=>"Tierra", fr=>"Terre", ru=>"Земля")
            }
            Text::Light => {
                i18n!(en=>"Light", sc=>"光明", tc=>"光明", jp=>"光", ko=>"빛", es=>"Luz", fr=>"Lumière", ru=>"Свет")
            }
            Text::Dark => {
                i18n!(en=>"Dark", sc=>"黑暗", tc=>"黑暗", jp=>"闇", ko=>"어둠", es=>"Oscuridad", fr=>"Ténèbres", ru=>"Тьма")
            }
            Text::Time => {
                i18n!(en=>"Time", sc=>"时间", tc=>"時間", jp=>"時間", ko=>"시간", es=>"Tiempo", fr=>"Temps", ru=>"Время")
            }
            Text::Space => {
                i18n!(en=>"Space", sc=>"空间", tc=>"空間", jp=>"空間", ko=>"공간", es=>"Espacio", fr=>"Espace", ru=>"Пространство")
            }
            Text::Power => {
                i18n!(en=>"Power", sc=>"力量", tc=>"力量", jp=>"力", ko=>"힘", es=>"Poder", fr=>"Pouvoir", ru=>"Сила")
            }
            Text::Speed => {
                i18n!(en=>"Speed", sc=>"速度", tc=>"速度", jp=>"速度", ko=>"속도", es=>"Velocidad", fr=>"Vitesse", ru=>"Скорость")
            }
            Text::Luck => {
                i18n!(en=>"Luck", sc=>"运气", tc=>"運氣", jp=>"運", ko=>"운", es=>"Suerte", fr=>"Chance", ru=>"Удача")
            }
            Text::Arrow => {
                i18n!(en=>"Arrow", sc=>"箭", tc=>"箭", jp=>"矢", ko=>"화살", es=>"Flecha", fr=>"Flèche", ru=>"Стрела")
            }
            Text::Banner => {
                i18n!(en=>"Banner", sc=>"旗帜", tc=>"旗幟", jp=>"旗印", ko=>"깃발", es=>"Estandarte", fr=>"Bannière", ru=>"Знамя")
            }
            Text::Cloak => {
                i18n!(en=>"Cloak", sc=>"斗篷", tc=>"斗篷", jp=>"マント", ko=>"망토", es=>"Capa", fr=>"Cape", ru=>"Плащ")
            }
            Text::Dagger => {
                i18n!(en=>"Dagger", sc=>"匕首", tc=>"匕首", jp=>"ダガー", ko=>"단검", es=>"Daga", fr=>"Dague", ru=>"Кинжал")
            }
            Text::Elixir => {
                i18n!(en=>"Elixir", sc=>"灵药", tc=>"靈藥", jp=>"エリクサー", ko=>"엘릭서", es=>"Elixir", fr=>"Élixir", ru=>"Эликсир")
            }
            Text::Forge => {
                i18n!(en=>"Forge", sc=>"锻造", tc=>"鍛造", jp=>"鍛冶", ko=>"대장간", es=>"Forja", fr=>"Forge", ru=>"Кузница")
            }
            Text::Goblin => {
                i18n!(en=>"Goblin", sc=>"哥布林", tc=>"哥布林", jp=>"ゴブリン", ko=>"고블린", es=>"Goblin", fr=>"Gobelin", ru=>"Гоблин")
            }
            Text::Herald => {
                i18n!(en=>"Herald", sc=>"传令官", tc=>"傳令官", jp=>"使者", ko=>"전령", es=>"Heraldo", fr=>"Héraut", ru=>"Глашатай")
            }
            Text::Illusion => {
                i18n!(en=>"Illusion", sc=>"幻象", tc=>"幻象", jp=>"幻覚", ko=>"환영", es=>"Ilusión", fr=>"Illusion", ru=>"Иллюзия")
            }
            Text::Jester => {
                i18n!(en=>"Jester", sc=>"小丑", tc=>"小丑", jp=>"道化師", ko=>"어릿광대", es=>"Bufón", fr=>"Bouffon", ru=>"Шут")
            }
            Text::Keystone => {
                i18n!(en=>"Keystone", sc=>"基石", tc=>"基石", jp=>"要石", ko=>"중심석", es=>"Clave", fr=>"Pierre angulaire", ru=>"Краеугольный камень")
            }
            Text::Labyrinth => {
                i18n!(en=>"Labyrinth", sc=>"迷宫", tc=>"迷宮", jp=>"迷路", ko=>"미로", es=>"Laberinto", fr=>"Labyrinthe", ru=>"Лабиринт")
            }
            Text::Mystic => {
                i18n!(en=>"Mystic", sc=>"秘术师", tc=>"秘術師", jp=>"神秘主義者", ko=>"신비주의자", es=>"Místico", fr=>"Mystique", ru=>"Мистик")
            }
            Text::Nexus => {
                i18n!(en=>"Nexus", sc=>"枢纽", tc=>"樞紐", jp=>"連结点", ko=>"넥서스", es=>"Nexo", fr=>"Nexus", ru=>"Связующий")
            }
            Text::Oath => {
                i18n!(en=>"Oath", sc=>"誓言", tc=>"誓言", jp=>"誓い", ko=>"서약", es=>"Juramento", fr=>"Serment", ru=>"Клятва")
            }
            Text::Pendant => {
                i18n!(en=>"Pendant", sc=>"吊坠", tc=>"吊墜", jp=>"ペンダント", ko=>"펜던트", es=>"Colgante", fr=>"Pendentif", ru=>"Кулон")
            }
            Text::Quiver => {
                i18n!(en=>"Quiver", sc=>"箭袋", tc=>"箭袋", jp=>"矢筒", ko=>"화살통", es=>"Carcaj", fr=>"Carquois", ru=>"Колчан")
            }
            Text::Rune => {
                i18n!(en=>"Rune", sc=>"符文", tc=>"符文", jp=>"ルーン", ko=>"룬", es=>"Runa", fr=>"Rune", ru=>"Руна")
            }
            Text::Sigil => {
                i18n!(en=>"Sigil", sc=>"符印", tc=>"符印", jp=>"印章", ko=>"싸인", es=>"Sigilo", fr=>"Sceau", ru=>"Печать")
            }
            Text::Tome => {
                i18n!(en=>"Tome", sc=>"典籍", tc=>"典籍", jp=>"魔導書", ko=>"고서", es=>"Tomo", fr=>"Tome", ru=>"Фолиант")
            }
            Text::Urn => {
                i18n!(en=>"Urn", sc=>"骨灰瓮", tc=>"骨灰甕", jp=>"壺", ko=>"항아리", es=>"Urna", fr=>"Urne", ru=>"Урна")
            }
            Text::Vial => {
                i18n!(en=>"Vial", sc=>"小瓶", tc=>"小瓶", jp=>"小瓶", ko=>"작은 병", es=>"Vial", fr=>"Fiole", ru=>"Флакон")
            }
            Text::Warlock => {
                i18n!(en=>"Warlock", sc=>"术士", tc=>"術士", jp=>"ウォーロック", ko=>"흑마법사", es=>"Brujo", fr=>"Démoniste", ru=>"Чернокнижник")
            }
            Text::Yew => {
                i18n!(en=>"Yew", sc=>"紫杉", tc=>"紫杉", jp=>"イチイ", ko=>"주목", es=>"Tejo", fr=>"If", ru=>"Тис")
            }
            Text::Zealot => {
                i18n!(en=>"Zealot", sc=>"狂热者", tc=>"狂熱者", jp=>"狂信者", ko=>"광신자", es=>"Fanático", fr=>"Zélote", ru=>"Фанатик")
            }
            Text::Altar => {
                i18n!(en=>"Altar", sc=>"祭坛", tc=>"祭壇", jp=>"祭壇", ko=>"제단", es=>"Altar", fr=>"Autel", ru=>"Алтарь")
            }
            Text::Brew => {
                i18n!(en=>"Brew", sc=>"酿造", tc=>"釀造", jp=>"醸造", ko=>"양조", es=>"Brebaje", fr=>"Breuvage", ru=>"Варево")
            }
            Text::Cairn => {
                i18n!(en=>"Cairn", sc=>"石冢", tc=>"石塚", jp=>"ケルン", ko=>"케언", es=>"Montón de piedras", fr=>"Cairn", ru=>"Курган")
            }
            Text::Druid => {
                i18n!(en=>"Druid", sc=>"德鲁伊", tc=>"德魯伊", jp=>"ドルイド", ko=>"드루이드", es=>"Druida", fr=>"Druide", ru=>"Друид")
            }
            Text::Effigy => {
                i18n!(en=>"Effigy", sc=>"雕像", tc=>"雕像", jp=>"肖像", ko=>"모형", es=>"Efigie", fr=>"Effigie", ru=>"Эффигия")
            }
            Text::Frost => {
                i18n!(en=>"Frost", sc=>"寒霜", tc=>"寒霜", jp=>"霜", ko=>"서리", es=>"Escarcha", fr=>"Givre", ru=>"Иней")
            }
            Text::Glyph => {
                i18n!(en=>"Glyph", sc=>"雕文", tc=>"雕文", jp=>"グリフ", ko=>"글리프", es=>"Glifo", fr=>"Glyphe", ru=>"Глиф")
            }
            Text::Hovel => {
                i18n!(en=>"Hovel", sc=>"茅屋", tc=>"茅屋", jp=>"小屋", ko=>"오두막", es=>"Choza", fr=>"Taudis", ru=>"Лачуга")
            }
            Text::Ire => {
                i18n!(en=>"Ire", sc=>"怒火", tc=>"怒火", jp=>"憤怒", ko=>"분노", es=>"Ira", fr=>"Courroux", ru=>"Гнев")
            }
            Text::Jolt => {
                i18n!(en=>"Jolt", sc=>"震动", tc=>"震動", jp=>"衝撃", ko=>"격동", es=>"Sacudida", fr=>"Secousse", ru=>"Толчок")
            }
            Text::Knoll => {
                i18n!(en=>"Knoll", sc=>"土丘", tc=>"土丘", jp=>"小丘", ko=>"언덕", es=>"Loma", fr=>"Butte", ru=>"Холмик")
            }
            Text::Lich => {
                i18n!(en=>"Lich", sc=>"巫妖", tc=>"巫妖", jp=>"リッチ", ko=>"리치", es=>"Liche", fr=>"Liche", ru=>"Лич")
            }
            Text::Maw => {
                i18n!(en=>"Maw", sc=>"巨口", tc=>"巨口", jp=>"顎", ko=>"아가리", es=>"Fauces", fr=>"Gueule", ru=>"Пасть")
            }
            Text::Nectar => {
                i18n!(en=>"Nectar", sc=>"花蜜", tc=>"花蜜", jp=>"神酒", ko=>"넥타르", es=>"Néctar", fr=>"Nectar", ru=>"Нектар")
            }
            Text::Omen => {
                i18n!(en=>"Omen", sc=>"预兆", tc=>"預兆", jp=>"前兆", ko=>"전조", es=>"Presagio", fr=>"Présage", ru=>"Предзнаменование")
            }
            Text::Plague => {
                i18n!(en=>"Plague", sc=>"瘟疫", tc=>"瘟疫", jp=>"疫病", ko=>"역병", es=>"Plaga", fr=>"Peste", ru=>"Чума")
            }
            Text::Quartz => {
                i18n!(en=>"Quartz", sc=>"石英", tc=>"石英", jp=>"石英", ko=>"석영", es=>"Cuarzo", fr=>"Quartz", ru=>"Кварц")
            }
            Text::Revenant => {
                i18n!(en=>"Revenant", sc=>"亡魂", tc=>"亡魂", jp=>"レブナント", ko=>"레버넌트", es=>"Espectro", fr=>"Revenant", ru=>"Ревенант")
            }
            Text::Shroud => {
                i18n!(en=>"Shroud", sc=>"裹尸布", tc=>"裹屍布", jp=>"死装束", ko=>"수의", es=>"Sudario", fr=>"Linceul", ru=>"Савван")
            }
            Text::Tundra => {
                i18n!(en=>"Tundra", sc=>"冻原", tc=>"凍原", jp=>"ツンドラ", ko=>"툰드라", es=>"Tundra", fr=>"Toundra", ru=>"Тундра")
            }
            Text::Undead => {
                i18n!(en=>"Undead", sc=>"亡灵", tc=>"亡靈", jp=>"アンデッド", ko=>"언데드", es=>"No-muerto", fr=>"Mort-vivant", ru=>"Нежить")
            }
            Text::Vortex => {
                i18n!(en=>"Vortex", sc=>"漩涡", tc=>"漩渦", jp=>"渦", ko=>"소용돌이", es=>"Vórtice", fr=>"Vortex", ru=>"Вихрь")
            }
            Text::Wight => {
                i18n!(en=>"Wight", sc=>"尸鬼", tc=>"屍鬼", jp=>"ワイト", ko=>"와이트", es=>"Espectro", fr=>"Goule", ru=>"Вайт")
            }
            Text::Xeric => {
                i18n!(en=>"Xeric", sc=>"干旱的", tc=>"乾旱的", jp=>"乾燥した", ko=>"건조한", es=>"Xérico", fr=>"Xérique", ru=>"Ксерофитный")
            }
            Text::Yonder => {
                i18n!(en=>"Yonder", sc=>"远处的", tc=>"遠處的", jp=>"彼方の", ko=>"저편의", es=>"Allá", fr=>"Là-bas", ru=>"Вон там")
            }
            Text::Eldritch => {
                i18n!(en=>"Eldritch", sc=>"诡异的", tc=>"詭異的", jp=>"不気味な", ko=>"기괴한", es=>"Misterioso", fr=>"Ésotérique", ru=>"Жуткий")
            }
            Text::Phylactery => {
                i18n!(en=>"Phylactery", sc=>"命匣", tc=>"命匣", jp=>"フィラクタリー", ko=>"필랙터리", es=>"Filacteria", fr=>"Phylactère", ru=>"Филактерия")
            }
            Text::Zephyr => {
                i18n!(en=>"Zephyr", sc=>"和风", tc=>"和風", jp=>"そよ風", ko=>"산들바람", es=>"Céfiro", fr=>"Zéphyr", ru=>"Зефир")
            }
            Text::PressStar => {
                i18n!(en=>"Press Start", sc=>"按下开始", tc=>"按下開始", jp=>"スタートを押す", ko=>"시작 버튼을 누르세요", es=>"Presiona Inicio", fr=>"Appuyez sur Démarrer", ru=>"Нажмите Старт")
            }
            Text::GameOver => {
                i18n!(en=>"Game Over", sc=>"游戏结束", tc=>"遊戲結束", jp=>"ゲームオーバー", ko=>"게임 오버", es=>"Juego Terminado", fr=>"Game Over", ru=>"Игра Окончена")
            }
            Text::Continue => {
                i18n!(en=>"Continue?", sc=>"继续吗？", tc=>"繼續嗎？", jp=>"続けますか？", ko=>"계속하시겠습니까？", es=>"¿Continuar?", fr=>"Continuer?", ru=>"Продолжить?")
            }
            Text::NewGame => {
                i18n!(en=>"New Game", sc=>"新游戏", tc=>"新遊戲", jp=>"ニューゲーム", ko=>"새 게임", es=>"Nuevo Juego", fr=>"Nouvelle Partie", ru=>"Новая Игра")
            }
            Text::HighScore => {
                i18n!(en=>"High Score", sc=>"最高分", tc=>"最高分", jp=>"ハイスコア", ko=>"최고 점수", es=>"Máxima Puntuación", fr=>"Meilleur Score", ru=>"Рекорд")
            }
            Text::SoundOnO => {
                i18n!(en=>"Sound On/Off", sc=>"声音开/关", tc=>"聲音開/關", jp=>"サウンド オン/オフ", ko=>"사운드 켜기/끄기", es=>"Sonido On/Off", fr=>"Son On/Off", ru=>"Звук Вкл/Выкл")
            }
            Text::FullScree => {
                i18n!(en=>"Full Screen", sc=>"全屏", tc=>"全螢幕", jp=>"フルスクリーン", ko=>"전체 화면", es=>"Pantalla Completa", fr=>"Plein Écran", ru=>"Полный Экран")
            }
            Text::VolumeUp => {
                i18n!(en=>"Volume Up/Down", sc=>"音量增加/减少", tc=>"音量增加/減少", jp=>"音量 上げ/下げ", ko=>"볼륨 높이기/낮추기", es=>"Subir/Bajar Volumen", fr=>"Volume +/-", ru=>"Громкость +/-")
            }
            Text::Checkpoint => {
                i18n!(en=>"Checkpoint Reached", sc=>"到达检查点", tc=>"到達檢查點", jp=>"チェックポイント到達", ko=>"체크포인트 도달", es=>"Punto de Control Alcanzado", fr=>"Point de Contrôle Atteint", ru=>"Контрольная Точка Достигнута")
            }
            Text::UnlockAch => {
                i18n!(en=>"Unlock Achievement", sc=>"解锁成就", tc=>"解鎖成就", jp=>"実績解除", ko=>"업적 해제", es=>"Logro Desbloqueado", fr=>"Succès Débloqué", ru=>"Достижение Разблокировано")
            }
            Text::Loading => {
                i18n!(en=>"Loading...", sc=>"加载中...", tc=>"載入中...", jp=>"ロード中...", ko=>"로드 중...", es=>"Cargando...", fr=>"Chargement...", ru=>"Загрузка...")
            }
            Text::PleaseWai => {
                i18n!(en=>"Please Wait", sc=>"请稍候", tc=>"請稍候", jp=>"お待ちください", ko=>"잠시 기다려 주세요", es=>"Por Favor Espere", fr=>"Veuillez Patienter", ru=>"Пожалуйста, Подождите")
            }
            Text::Connection => {
                i18n!(en=>"Connection Lost", sc=>"连接丢失", tc=>"連接丟失", jp=>"接続切断", ko=>"연결 끊김", es=>"Conexión Perdida", fr=>"Connexion Perdue", ru=>"Соединение Потеряно")
            }
            Text::Retry => {
                i18n!(en=>"Retry?", sc=>"重试？", tc=>"重試？", jp=>"再試行しますか？", ko=>"다시 시도하시겠습니까？", es=>"¿Reintentar?", fr=>"Réessayer?", ru=>"Повторить?")
            }
            Text::ConfirmDe => {
                i18n!(en=>"Confirm Delete", sc=>"确认删除", tc=>"確認刪除", jp=>"削除確認", ko=>"삭제 확인", es=>"Confirmar Eliminación", fr=>"Confirmer la Suppression", ru=>"Подтвердить Удаление")
            }
            Text::NotEnough => {
                i18n!(en=>"Not Enough Gold", sc=>"金币不足", tc=>"金幣不足", jp=>"ゴールド不足", ko=>"골드 부족", es=>"No Hay Suficiente Oro", fr=>"Or Insuffisant", ru=>"Недостаточно Золота")
            }
            Text::LevelUp => {
                i18n!(en=>"Level Up!", sc=>"升级！", tc=>"升級！", jp=>"レベルアップ！", ko=>"레벨 업！", es=>"¡Subir de Nivel!", fr=>"Niveau Supérieur!", ru=>"Повышение Уровня!")
            }
            Text::CriticalH => {
                i18n!(en=>"Critical Hit!", sc=>"暴击！", tc=>"暴擊！", jp=>"クリティカルヒット！", ko=>"치명타!", es=>"¡Golpe Crítico!", fr=>"Coup Critique!", ru=>"Критический Удар!")
            }
            Text::OutofMan => {
                i18n!(en=>"Out of Mana", sc=>"魔力耗尽", tc=>"魔力耗盡", jp=>"マナ不足", ko=>"마나 부족", es=>"Sin Maná", fr=>"Plus de Mana", ru=>"Закончилась Мана")
            }
            Text::PartyFull => {
                i18n!(en=>"Party Full", sc=>"队伍已满", tc=>"隊伍已滿", jp=>"パーティー満員", ko=>"파티 정원 초과", es=>"Equipo Lleno", fr=>"Groupe Complet", ru=>"Группа Заполнена")
            }
            Text::Welcometo => {
                i18n!(en=>"Welcome to the game!", sc=>"欢迎来到游戏！", tc=>"歡迎來到遊戲！", jp=>"ゲームへようこそ！", ko=>"게임에 오신 것을 환영합니다!", es=>"¡Bienvenido al juego!", fr=>"Bienvenue dans le jeu!", ru=>"Добро пожаловать в игру!")
            }
            Text::Youfound => {
                i18n!(en=>"You found a hidden treasure!", sc=>"你发现了隐藏宝藏！", tc=>"你發現了隱藏寶藏！", jp=>"隠し宝を見つけた！", ko=>"숨겨진 보물을 발견했습니다!", es=>"¡Encontraste un tesoro oculto!", fr=>"Vous avez trouvé un trésor caché!", ru=>"Вы нашли скрытое сокровище!")
            }
            Text::Theenemy => {
                i18n!(en=>"The enemy is approaching!", sc=>"敌人正在接近！", tc=>"敵人正在接近！", jp=>"敵が近づいている！", ko=>"적이 접근 중입니다!", es=>"¡El enemigo se acerca!", fr=>"L'ennemi approche!", ru=>"Враг приближается!")
            }
            Text::Yourhealt => {
                i18n!(en=>"Your health is low.", sc=>"你的生命值过低。", tc=>"你的生命值過低。", jp=>"体力が少ないです。", ko=>"체력이 낮습니다.", es=>"Tu salud es baja.", fr=>"Votre santé est faible.", ru=>"У вас мало здоровья.")
            }
            Text::Wouldyou => {
                i18n!(en=>"Would you like to save?", sc=>"要保存吗？", tc=>"要儲存嗎？", jp=>"セーブしますか？", ko=>"저장하시겠습니까?", es=>"¿Quieres guardar?", fr=>"Voulez-vous sauvegarder?", ru=>"Хотите сохраниться?")
            }
            Text::Thisdoor => {
                i18n!(en=>"This door is locked.", sc=>"这扇门锁上了。", tc=>"這扇門鎖上了。", jp=>"このドアは鍵がかかっている。", ko=>"이 문은 잠겨 있습니다.", es=>"Esta puerta está cerrada.", fr=>"Cette porte est verrouillée.", ru=>"Эта дверь заперта.")
            }
            Text::Youneeda => {
                i18n!(en=>"You need a key to open it.", sc=>"你需要钥匙才能打开。", tc=>"你需要鑰匙才能打開。", jp=>"開けるには鍵が必要だ。", ko=>"열려면 열쇠가 필요합니다.", es=>"Necesitas una llave para abrirla.", fr=>"Vous avez besoin d'une clé pour l'ouvrir.", ru=>"Вам нужен ключ, чтобы открыть.")
            }
            Text::Amysterio => {
                i18n!(en=>"A mysterious voice whispers...", sc=>"神秘的声音低语...", tc=>"神秘的聲音低語...", jp=>"謎の声が囁く...", ko=>"신비로운 목소리가 속삭입니다...", es=>"Una voz misteriosa susurra...", fr=>"Une voix mystérieuse murmure...", ru=>"Таинственный голос шепчет...")
            }
            Text::Thebattle => {
                i18n!(en=>"The battle begins!", sc=>"战斗开始！", tc=>"戰鬥開始！", jp=>"戦闘開始！", ko=>"전투 시작!", es=>"¡La batalla comienza!", fr=>"Le combat commence!", ru=>"Битва начинается!")
            }
            Text::Youdefeat => {
                i18n!(en=>"You defeated the boss!", sc=>"你击败了首领！", tc=>"你擊敗了首領！", jp=>"ボスを倒した！", ko=>"보스를 물리쳤습니다!", es=>"¡Derrotaste al jefe!", fr=>"Vous avez vaincu le boss!", ru=>"Вы победили босса!")
            }
            Text::Criticalh => {
                i18n!(en=>"Critical hit! Double damage dealt!", sc=>"暴击！造成双倍伤害！", tc=>"暴擊！造成雙倍傷害！", jp=>"クリティカルヒット！ダメージ2倍！", ko=>"치명타! 2배 피해!", es=>"¡Golpe crítico! ¡Doble daño!", fr=>"Coup critique ! Dégâts doublés !", ru=>"Критический удар! Двойной урон!")
            }
            Text::Enemyiss => {
                i18n!(en=>"Enemy is stunned for 3 seconds.", sc=>"敌人被眩晕3秒。", tc=>"敵人被眩暈3秒。", jp=>"敵は3秒間気絶した。", ko=>"적이 3초간 기절했습니다.", es=>"El enemigo está aturdido por 3 segundos.", fr=>"L'ennemi est étourdi pendant 3 secondes.", ru=>"Враг оглушён на 3 секунды.")
            }
            Text::Yourshiel => {
                i18n!(en=>"Your shield blocked 80% of the damage.", sc=>"你的盾牌格挡了80%伤害。", tc=>"你的盾牌格擋了80%傷害。", jp=>"盾でダメージの80%を防いだ。", ko=>"방패로 피해의 80%를 막았습니다.", es=>"Tu escudo bloqueó el 80% del daño.", fr=>"Votre bouclier a bloqué 80% des dégâts.", ru=>"Ваш щит блокировал 80% урона.")
            }
            Text::Ahiddenp => {
                i18n!(en=>"A hidden path reveals itself!", sc=>"一条隐藏路径出现了！", tc=>"一條隱藏路徑出現了！", jp=>"隠し道が現れた！", ko=>"숨겨진 길이 나타났습니다!", es=>"¡Un camino oculto se revela!", fr=>"Un passage secret apparaît !", ru=>"Скрытый путь открылся!")
            }
            Text::Yousense => {
                i18n!(en=>"You sense powerful magic nearby.", sc=>"你感应到附近有强大魔法。", tc=>"你感應到附近有強大魔法。", jp=>"近くに強力な魔法を感知した。", ko=>"근처에 강력한 마법이 느껴집니다.", es=>"Sientes magia poderosa cerca.", fr=>"Vous sentez une magie puissante à proximité.", ru=>"Вы чувствуете мощную магию рядом.")
            }
            Text::Newskill => {
                i18n!(en=>"New skill unlocked: Fireball!", sc=>"新技能已解锁：火球术！", tc=>"新技能已解鎖：火球術！", jp=>"新スキル解放：ファイアボール！", ko=>"새 스킬 해제: 파이어볼!", es=>"¡Nueva habilidad desbloqueada: Bola de fuego!", fr=>"Nouvelle compétence débloquée : Boule de feu !", ru=>"Новый навык: Огненный шар!")
            }
            Text::Strengt => {
                i18n!(en=>"+5 Strength after training!", sc=>"训练后力量+5！", tc=>"訓練後力量+5！", jp=>"トレーニングで筋力+5！", ko=>"훈련 후 힘 +5!", es=>"¡+5 Fuerza después del entrenamiento!", fr=>"+5 Force après l'entraînement !", ru=>"+5 к силе после тренировки!")
            }
            Text::Yourreput => {
                i18n!(en=>"Your reputation with the guild has improved.", sc=>"你与公会的声望提升了。", tc=>"你與公會的聲望提升了。", jp=>"ギルドとの好感度が上がった。", ko=>"길드와의 평판이 향상되었습니다.", es=>"Tu reputación con el gremio ha mejorado.", fr=>"Votre réputation avec la guilde s'est améliorée.", ru=>"Ваша репутация с гильдией повысилась.")
            }
            Text::Autosave => {
                i18n!(en=>"Auto-save complete. Press [F5] to quick load.", sc=>"自动保存完成。按[F5]快速读取。", tc=>"自動儲存完成。按[F5]快速讀取。", jp=>"自動セーブ完了。[F5]でクイックロード。", ko=>"자동 저장 완료. [F5]로 빠르게 불러오기.", es=>"Guardado automático completo. Presiona [F5] para cargar rápido.", fr=>"Sauvegarde auto terminée. Appuyez sur [F5] pour charger rapidement.", ru=>"Автосохранение завершено. Нажмите [F5] для быстрой загрузки.")
            }
            Text::Controller => {
                i18n!(en=>"Controller disconnected. Reconnect to continue.", sc=>"控制器已断开，请重新连接。", tc=>"控制器已斷開，請重新連接。", jp=>"コントローラーが切断されました。", ko=>"컨트롤러 연결 끊김. 재연결하세요.", es=>"Controlador desconectado. Reconecta para continuar.", fr=>"Manette déconnectée. Reconnectez-vous.", ru=>"Контроллер отключён. Переподключите.")
            }
            Text::Downloadin => {
                i18n!(en=>"Downloading update (72%)... Do not turn off.", sc=>"正在下载更新(72%)... 请勿关闭。", tc=>"正在下載更新(72%)... 請勿關閉。", jp=>"更新をダウンロード中(72%)... 終了しないでください。", ko=>"업데이트 다운로드 중(72%)... 종료하지 마세요.", es=>"Descargando actualización (72%)... No apagues.", fr=>"Téléchargement de la mise à jour (72%)... Ne pas éteindre.", ru=>"Загрузка обновления (72%)... Не выключайте.")
            }
            Text::Theforest => {
                i18n!(en=>"The forest is cursed. Travel at your own risk.", sc=>"“森林被诅咒了，前行需谨慎。”", tc=>"「森林被詛咒了，前行需謹慎。」", jp=>"「森は呪われている。行くなら自己責任で」", ko=>"숲은 저주받았습니다. 가지 마십시오.", es=>"El bosque está maldito. Viaja bajo tu propio riesgo.", fr=>"La forêt est maudite. Aventurez-vous à vos risques et périls.", ru=>"«Лес проклят. Путешествуйте на свой страх и риск.»")
            }
            Text::Illtrade => {
                i18n!(en=>"I'll trade you 3 herbs for a healing potion.", sc=>"“用3株草药换一瓶治疗药水。”", tc=>"「用3株草藥換一瓶治療藥水。」", jp=>"「回復ポーションと3つの薬草を交換しよう」", ko=>"치유 물약과 약초 3개를 교환하겠습니다.", es=>"Te cambiaré 3 hierbas por una poción de curación.", fr=>"Je t'échange 3 herbes contre une potion de soin.", ru=>"«Я обменяю 3 травы на зелье лечения.»")
            }
            Text::Bewarethe => {
                i18n!(en=>"Beware the red moon – monsters grow stronger!", sc=>"“当心红月，怪物会变强！”", tc=>"「當心紅月，怪物會變強！」", jp=>"「赤い月の夜は魔物が強くなるぞ！」", ko=>"붉은 달 밤에는 몬스터가 강해집니다!", es=>"¡Cuidado con la luna roja, los monstruos se fortalecen!", fr=>"Méfiez-vous de la lune rouge – les monstres deviennent plus forts !", ru=>"«Бойтесь красной луны — монстры станут сильнее!»")
            }
            Text::Questupda => {
                i18n!(en=>"Quest updated: Find the missing blacksmith.", sc=>"任务更新：寻找失踪的铁匠。", tc=>"任務更新：尋找失蹤的鐵匠。", jp=>"クエスト更新：行方不明の鍛冶屋を探せ。", ko=>"퀘스트 업데이트: 실종된 대장장이를 찾으세요.", es=>"Misión actualizada: Encuentra al herrero desaparecido.", fr=>"Quête mise à jour : Trouvez le forgeron disparu.", ru=>"Задание обновлено: Найдите пропавшего кузнеца.")
            }
            Text::Optionalo => {
                i18n!(en=>"Optional objective: Collect 10 wolf fangs.", sc=>"可选目标：收集10颗狼牙。", tc=>"可選目標：收集10顆狼牙。", jp=>"オプション目標：狼の牙を10本集めろ。", ko=>"선택 목표: 늑대 이빨 10개 수집.", es=>"Objetivo opcional: Recolecta 10 colmillos de lobo.", fr=>"Objectif facultatif : Ramassez 10 crocs de loup.", ru=>"Дополнительная цель: Соберите 10 волчьих клыков.")
            }
            Text::Timeremai => {
                i18n!(en=>"Time remaining: 14 minutes 32 seconds.", sc=>"剩余时间：14分32秒。", tc=>"剩餘時間：14分32秒。", jp=>"残り時間：14分32秒。", ko=>"남은 시간: 14분 32초.", es=>"Tiempo restante: 14 minutos 32 segundos.", fr=>"Temps restant : 14 minutes 32 secondes.", ru=>"Осталось времени: 14 минут 32 секунды.")
            }
            Text::Iusedto => {
                i18n!(en=>"I used to be an adventurer like you... then I took a potato to the knee.", sc=>"“我以前和你一样是个冒险者…直到我的膝盖中了一颗土豆。”", tc=>"「我以前和你一樣是個冒險者…直到我的膝蓋中了一顆馬鈴薯。」", jp=>"「お前のような冒険者だったが…ジャガイモを膝に受けてな」", ko=>"너 같은 모험가였는데... 무릎에 감자맞고 말았지.", es=>"Yo también fui aventurero... hasta que una patata me golpeó la rodilla.", fr=>"J'étais un aventurier comme toi... puis j'ai pris une patate dans le genou.", ru=>"«Я был авантюристом, как ты... пока не получил картошкой в колено.»")
            }
            Text::Thisisnt => {
                i18n!(en=>"This isn't even my final form!", sc=>"“这还不是我的最终形态！”", tc=>"「這還不是我的最終形態！」", jp=>"「これが俺の真の姿じゃない！」", ko=>"이건 내 최종 형태가 아니야!", es=>"¡Esta ni siquiera es mi forma final!", fr=>"Ce n'est même pas ma forme finale !", ru=>"«Это ещё не моя окончательная форма!»")
            }
            Text::Theancien => {
                i18n!(en=>"The ancient prophecy foretold that a hero would rise to save the world from darkness.", sc=>"古老的预言预示一位英雄将崛起，从黑暗中拯救世界。", tc=>"古老的預言預示一位英雄將崛起，從黑暗中拯救世界。", jp=>"古代の予言は、英雄が現れ闇から世界を救うと告げていた。", ko=>"고대의 예언은 영웅이 나타나 어둠으로부터 세상을 구할 것이라고 예고했습니다.", es=>"La antigua profecía decía que un héroe se alzaría para salvar al mundo de la oscuridad.", fr=>"L'ancienne prophétie annonçait qu'un héros se lèverait pour sauver le monde des ténèbres.", ru=>"Древнее пророчество гласило, что герой восстанет, чтобы спасти мир от тьмы.")
            }
            Text::Tounlock => {
                i18n!(en=>"To unlock this ability, you must first complete the trial of courage.", sc=>"要解锁此能力，你必须先完成勇气试炼。", tc=>"要解鎖此能力，你必須先完成勇氣試煉。", jp=>"この能力を解放するには、まず勇気の試練をクリアしなければならない。", ko=>"이 능력을 해제하려면 먼저 용기의 시련을 완료해야 합니다.", es=>"Para desbloquear esta habilidad, primero debes completar la prueba de valentía.", fr=>"Pour débloquer cette capacité, vous devez d'abord accomplir l'épreuve du courage.", ru=>"Чтобы разблокировать эту способность, вы должны сначала пройти испытание храбрости.")
            }
            Text::Thekingdo => {
                i18n!(en=>"The kingdom of Eldoria was once a land of peace and prosperity, but now it lies in ruins under the rule of the Dark Sorcerer. Legends speak of a forgotten artifact hidden deep within the Crystal Caves, said to hold the power to restore balance. Will you embark on this perilous journey to reclaim the kingdom's lost glory?", sc=>"埃尔多利亚王国曾是一片和平繁荣的土地，但现在它在黑暗巫师的统治下沦为废墟。传说水晶洞穴深处藏有一件被遗忘的神器，据说拥有恢复平衡的力量。你愿意踏上这段危险的旅程，夺回王国失去的荣耀吗？", tc=>"埃爾多利亞王國曾是一片和平繁榮的土地，但現在它在黑暗巫師的統治下淪為廢墟。傳說水晶洞穴深處藏有一件被遺忘的神器，據說擁有恢復平衡的力量。你願意踏上這段危險的旅程，奪回王國失去的榮耀嗎？", jp=>"エルドリア王国はかつて平和と繁栄の土地でしたが、今は闇の魔法使いの支配下で廃墟と化しています。伝説によると、クリスタルケイブの奥深くに忘れられた遺物が隠されており、バランスを回復する力を持つと言われています。あなたはこの危険な旅に乗り出し、王国の失われた栄光を取り戻しますか？", ko=>"엘도리아 왕국은 한때 평화와 번영의 땅이었지만, 이제는 다크 소서러의 통치 아래 폐허가 되었습니다. 전설에 따르면 크리스탈 동굴 깊은 곳에 잊혀진 유물이 숨겨져 있으며, 균형을 회복할 수 있는 힘을 지니고 있다고 합니다. 왕국의 잃어버린 영광을 되찾기 위해 이 위험한 여정에 떠나시겠습니까?", es=>"El reino de Eldoria fue una vez una tierra de paz y prosperidad, pero ahora yace en ruinas bajo el gobierno del Hechicero Oscuro. Las leyendas hablan de un artefacto olvidado escondido en lo profundo de las Cuevas de Cristal, que se dice que tiene el poder de restaurar el equilibrio. ¿Te embarcarás en este peligroso viaje para reclamar la gloria perdida del reino?", fr=>"Le royaume d'Eldoria était autrefois une terre de paix et de prospérité, mais il gît maintenant en ruines sous le règle du Sorcier Noir. Les légendes évoquent un artefact oublié caché au plus profond des Grottes de Cristal, qui aurait le pouvoir de rétablir l'équilibre. Vous lancerez-vous dans ce périlleux voyage pour reconquérir la gloire perdue du royaume ?", ru=>"Королевство Эльдория когда-то было землей мира и процветания, но теперь оно лежит в руинах под властью Темного Колдуна. Легенды говорят о забытом артефакте, спрятанном глубоко в Хрустальных Пещерах, который, как говорят, обладает силой восстановить баланс. Отправитесь ли вы в это опасное путешествие, чтобы вернуть утраченную славу королевства?")
            }
        }
    }
}
