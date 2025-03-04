import { Input, Select, Switch, Tooltip } from 'antd';
import { ChangeEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSelector } from 'react-redux';

import { startup } from '../../../shared/tauri/infra';
import { useAppDispatch } from '../../../shared/utils/infra';

import { RootActions } from '../../../shared/store/app/reducer';
import { RootSelectors } from '../../../shared/store/app/selectors';
import { Icon } from 'src/apps/shared/components/Icon';

import { LanguageList } from '../../../../../shared/lang';
import { SettingsGroup, SettingsOption } from '../../../../components/SettingsBox';
import { Colors } from './Colors';
import { IconPacks } from './IconPacks';
import { Themes } from './Themes';

export function General() {
  const [changingAutostart, setChangingAutostart] = useState(false);

  const autostartStatus = useSelector(RootSelectors.autostart);
  const language = useSelector(RootSelectors.language);
  const dateFormat = useSelector(RootSelectors.dateFormat);

  const { t } = useTranslation();
  const dispatch = useAppDispatch();

  const onAutoStart = async (value: boolean) => {
    setChangingAutostart(true);
    if (value) {
      await startup.enable();
    } else {
      await startup.disable();
    }
    setChangingAutostart(false);
    dispatch(RootActions.setAutostart(value));
  };

  const onDateFormatChange = (e: ChangeEvent<HTMLInputElement>) =>
    dispatch(RootActions.setDateFormat(e.target.value));

  return (
    <>
      <SettingsGroup>
        <SettingsOption>
          <b>{t('general.startup')}</b>
          <Switch
            onChange={onAutoStart}
            value={!!autostartStatus}
            loading={changingAutostart || autostartStatus === null}
          />
        </SettingsOption>
      </SettingsGroup>
      <SettingsGroup>
        <SettingsOption>
          <b>{t('general.language')}</b>
          <Select
            showSearch
            optionFilterProp="label"
            style={{ width: '200px' }}
            value={language}
            options={[...LanguageList]}
            onSelect={(value) => dispatch(RootActions.setLanguage(value))}
          />
        </SettingsOption>
        <SettingsOption>
          <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
            <b>{t('general.date_format')}</b>
            <Tooltip
              title={
                <a href="https://momentjs.com/docs/#/displaying/format/" target="_blank">
                  https://momentjs.com/docs/#/displaying/format/
                </a>
              }
            >
              <Icon iconName="LuHelpCircle" />
            </Tooltip>
          </div>
          <Input
            style={{ width: '200px', maxWidth: '200px' }}
            placeholder="YYYY-MM-DD"
            value={dateFormat}
            onChange={onDateFormatChange}
          />
        </SettingsOption>
      </SettingsGroup>

      <Colors />

      <SettingsGroup>
        <div style={{ marginBottom: '6px' }}>
          <b>{t('general.theme.label')}</b>
        </div>
        <Themes />
      </SettingsGroup>

      <SettingsGroup>
        <div style={{ marginBottom: '6px' }}>
          <b>{t('general.icon_pack.label')}</b>
        </div>
        <IconPacks />
      </SettingsGroup>
    </>
  );
}
