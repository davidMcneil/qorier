import { CourierState } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { NewSubscription } from "./new_subscription";
import { Pull } from "./pull";
import { SubscriptionsTable } from "./subscriptions_table";

interface Props {
  visible: boolean;
  courierState: CourierState;
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function SubscriptionsTab(props: Props) {
  return (
    <div class={props.visible ? "" : "is-hidden"}>
      <section class="section">
        <div class="container">
          <div class="columns">
            <div class="column">
              <NewSubscription setNotification={props.setNotification} />
            </div>
            <div class="column">
              <Pull setNotification={props.setNotification} />
            </div>
          </div>
        </div>
      </section>

      <section class="section">
        <div class="container">
          <SubscriptionsTable
            subscriptions={Array.from(props.courierState.subscriptionMap.values())}
            setNotification={props.setNotification}
            setDeleteConfirmation={props.setDeleteConfirmation}
          />
        </div>
      </section>
    </div>
  );
}
