import { NotificationsTM } from '../../../../shared/schemas/Placeholders';
import { Notifications } from './Notifications';
import { emit } from '@tauri-apps/api/event';
import { Popover } from 'antd';
import { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';

import { Item } from '../../item/infra';
import { useAppBlur } from '../../shared/hooks/infra';

import { Selectors } from '../../shared/store/app';

import { RootState } from '../../shared/store/domain';

interface Props {
  module: NotificationsTM;
}

export function NotificationsModule({ module }: Props) {
  const [openPreview, setOpenPreview] = useState(false);
  const count = useSelector((state: RootState) => Selectors.notifications(state).length);

  useAppBlur(() => {
    setOpenPreview(false);
  });

  useEffect(() => {
    emit('register-notifications-events');
  }, []);

  return (
    <Popover
      open={openPreview}
      trigger="click"
      onOpenChange={setOpenPreview}
      arrow={false}
      content={<Notifications />}
    >
      <Item extraVars={{ count }} module={module} />
    </Popover>
  );
}